use core::arch::global_asm;

use riscv::register::{
    scause::{self, Exception, Interrupt, Trap},
    stval, stvec,
    utvec::TrapMode,
};

use crate::{
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

#[no_mangle]
pub fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
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
            warn!("Illegal instruction found in app. Kernel killed it.");
            sys_exit(1)
        }
        cause => {
            panic!("unsupported trap {:?}, stval = {:#x}!", cause, stval)
        }
    }
    TASK_MANAGER.leave_trap();
    cx
}

pub fn enable_timer_interrupt() {
    unsafe { riscv::register::sie::set_stimer() }
}
