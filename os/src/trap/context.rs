use bit_field::BitField;
use riscv::register::sstatus::Sstatus;

#[repr(C)]
pub struct TrapContext {
    // field的顺序对应保存在栈的顺序
    // 所以用的repr(C) ?
    pub x: [usize; 32],
    pub sstatus: Sstatus,
    pub sepc: usize,
}

impl TrapContext {
    // 这个只是更改在内存里面对应的变量，实际上不会修改 sp
    pub fn set_sp(&mut self, sp: usize) {
        self.x[2] = sp;
    }

    pub unsafe fn new(entry: usize, sp: usize) -> Self {
        let mut sstatus = riscv::register::sstatus::read();
        // riscv crate 的 sstatus 变量不可修改bits
        // 用指针来 hack 一下
        let sstatus_ptr = &mut sstatus as *mut _ as *mut usize;
        if let Some(sstatus) = sstatus_ptr.as_mut() {
            // https://github.com/rcore-os/riscv/blob/master/src/register/sstatus.rs#L116
            sstatus.set_bit(8, false);
        }

        let mut ret = Self {
            x: [0; 32],
            sstatus,
            sepc: entry,
        };
        ret.set_sp(sp);
        ret
    }
}
