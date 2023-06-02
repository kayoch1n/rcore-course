use core::arch::{asm, global_asm};

use riscv::register::{
    scause::{self, Exception, Interrupt, Trap},
    sepc, stval, stvec,
    utvec::TrapMode,
};

use crate::{
    config::{TRAMPOLINE, TRAP_CONTEXT},
    syscall::{self, proc::sys_exit},
    task::{suspend_and_run_next, TASK_MANAGER},
    timer::set_next_trigger,
    warn,
};

pub use self::context::TrapContext;

mod context;

global_asm!(include_str!("trap.asm"));

/// 把 __all_traps 符号的地址写入到 stvec
pub fn init() {
    extern "C" {
        fn __all_traps();
    }
    unsafe { stvec::write(__all_traps as usize, TrapMode::Direct) };
}

pub fn set_kernel_trap_entry() {
    unsafe { stvec::write(kernel_trap_entry as usize, TrapMode::Direct) };
}

pub fn set_user_trap_entry() {
    // 要用 __all_traps 的虚拟地址，所以是 TRAMPOLINE
    unsafe { stvec::write(TRAMPOLINE, TrapMode::Direct) }
}

/// 不支持OS trap
#[no_mangle]
pub fn kernel_trap_entry() {
    panic!("kernel trap is not implemented")
}

#[no_mangle]
pub fn trap_handler() -> ! {
    // 修改 stvec 为 OS trap 入口
    set_kernel_trap_entry();

    let cx = TASK_MANAGER.get_current_trap_context();

    TASK_MANAGER.enter_trap();

    let scause = scause::read();
    let stval = stval::read();

    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            cx.sepc += 4;
            cx.x[10] = syscall::syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]) as usize;
        }
        Trap::Interrupt(Interrupt::SupervisorTimer) => {
            set_next_trigger();
            suspend_and_run_next();
        }
        Trap::Exception(Exception::StoreFault) | Trap::Exception(Exception::StorePageFault) => {
            warn!("Page fault found in app. Kernel killed it.");
            // TASK_MANAGER.run_next_app()
            sys_exit(1)
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            let address = sepc::read();
            warn!(
                "Illegal instruction found at {:#x}. Kernel killed it.",
                address
            );
            sys_exit(1)
        }
        cause => {
            panic!("unsupported trap {:?}, stval = {:#x}!", cause, stval)
        }
    }
    TASK_MANAGER.leave_trap();

    trap_return()
}

#[no_mangle]
pub fn trap_return() -> ! {
    // 恢复 app trap 入口
    set_user_trap_entry();

    extern "C" {
        fn __all_traps();
        fn __restore();
    }

    let restore_va: usize = __restore as usize - __all_traps as usize + TRAMPOLINE;
    let user_satp = TASK_MANAGER.get_current_token();

    unsafe {
        asm!(
            "fence.i",
            "jr {restore_va}",
            restore_va = in(reg) restore_va,
            in("a0") TRAP_CONTEXT,
            in("a1") user_satp, // 要将切换到 app 的 page table
        );
    }

    panic!("unreachable")
}

pub fn enable_timer_interrupt() {
    unsafe { riscv::register::sie::set_stimer() }
}
