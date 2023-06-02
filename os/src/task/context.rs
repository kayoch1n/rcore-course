use crate::trap::trap_return;

/// rcore借助trap来切换task context，
/// 因此 ra 和 sp 都是内核相关的地址
#[derive(Clone, Copy, Debug)]
#[repr(C)] // ?
pub struct TaskContext {
    /// OS的指令的地址，见 ```rcore::task::__switch```
    ra: usize,
    /// 在OS栈上的地址，见 ```rcore::task::__switch```
    sp: usize,
    /// sX 寄存器，见 ```rcore::task::__switch```
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

    /// 所有task第一次运行之前的 context
    /// ra 是 trap_return
    /// kernel_stack_bottom 是栈底，用作初始的sp
    pub fn init(kernel_stack_bottom: usize) -> Self {
        TaskContext {
            ra: trap_return as usize,
            sp: kernel_stack_bottom,
            s: [0; 12],
        }
    }
}
