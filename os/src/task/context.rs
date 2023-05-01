/// rcore借助trap来切换task context，
/// 因此 ra 和 sp 都是内核相关的地址
#[derive(Clone, Copy)]
#[repr(C)] // ?
pub struct TaskContext {
    /// 内核的指令的地址
    ra: usize,
    /// 在内核栈上的地址
    sp: usize,
    s: [usize; 12],
}

impl TaskContext {
    pub fn zero_init() -> Self {
        TaskContext {
            ra: 0,
            sp: 0,
            s: [0; 12],
        }
    }
    /// 使用给定的内核栈的地址作为栈顶、创建一个 task context。
    pub fn init(kernel_sp: usize) -> Self {
        extern "C" {
            fn __restore();
        }
        Self {
            ra: __restore as usize,
            sp: kernel_sp,
            s: [0; 12],
        }
    }
}
