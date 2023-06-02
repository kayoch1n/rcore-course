use bit_field::BitField;
use riscv::register::sstatus::Sstatus;

#[repr(C)]
pub struct TrapContext {
    // field的顺序对应保存在栈的顺序
    // 所以用的repr(C) ?
    pub x: [usize; 32],
    /// 发生 trap 前的特权等级
    pub sstatus: Sstatus,
    /// 发生 trap 前执行的最后一条指令的地址
    pub sepc: usize,

    // 为什么sscratch要指向一个 trap context 而不是 kernel stack
    // 因为要额外存储两个信息：kernel satp and trap handler
    // 只有一个 sscratch 根本不够用，所以把这几个值放到内存，
    pub kernel_satp: usize,
    /// OS栈的地址，是不是每个app的kernel_sp都是相同的呢？
    pub kernel_sp: usize,
    /// trap_handler 虚拟地址
    pub trap_handler: usize,
}

impl TrapContext {
    // 这个只是更改在内存里面对应的变量，实际上不会修改 sp
    pub fn set_sp(&mut self, sp: usize) {
        self.x[2] = sp;
    }

    /// 用给定的入口地址和栈顶创建一个 trap context。rcore 里面有两种方式保存 trap context：
    ///
    /// 1. trap.asm 内核响应 ecall trap 的一般方法 __all_traps
    ///
    /// 2. TrapContext::init 仅用于第一次执行 app 之前的准备工作。目的是复用 __restore
    ///
    /// entry 入口地址
    ///
    /// sp 用户栈
    pub fn init(
        entry: usize,
        sp: usize,
        kernel_satp: usize,
        kernel_sp: usize,
        trap_handler: usize,
    ) -> Self {
        let mut sstatus = riscv::register::sstatus::read();
        // riscv crate 的 sstatus 变量不可修改bits
        // 用指针来 hack 一下
        let sstatus_ptr = &mut sstatus as *mut _ as *mut usize;
        if let Some(sstatus) = unsafe { sstatus_ptr.as_mut() } {
            // 将sstatus.spp设置为user
            // https://github.com/rcore-os/riscv/blob/master/src/register/sstatus.rs#L116
            sstatus.set_bit(8, false);
            // 将sstatus.fs设置为1
            // https://five-embeddev.com/riscv-isa-manual/latest/machine.html#machine-status-registers-mstatus-and-mstatus
            // https://github.com/riscv-software-src/riscv-isa-sim/issues/221#issuecomment-407850084
            sstatus.set_bits(13..15, 1);
        }

        let mut ret = Self {
            x: [0; 32],
            sstatus,
            sepc: entry,
            kernel_satp,
            kernel_sp,
            trap_handler,
        };
        ret.set_sp(sp);
        ret
    }
}
