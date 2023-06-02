/// 8KiB
pub const USER_STACK_SIZE: usize = PAGESIZE * 2;
/// 8KiB
pub const KERNEL_STACK_SIZE: usize = PAGESIZE * 2;
// PS: 调大这个值会有bug导致在loader::init加载第一个app的时候
// 陷入死循环。调试发现在 __all_traps 处的内容也就是代码被覆盖掉了，变成一些垃圾数据
// pub const MAX_APP_NUM: usize = 64;

/// QEMU 时钟频率?
pub const CLOCK_FREQ: usize = 10000000;

pub const KERNEL_HEAP_SIZE: usize = 0x30_0000;

pub const PAGESIZE_BITS: usize = 12;
pub const PAGESIZE: usize = 1 << PAGESIZE_BITS;
pub const PAGEOFFSET_MASK: usize = PAGESIZE - 1;
pub const PAGETABLE_SIZE: usize = 512;

/// app 和 OS 的跳板 page
pub const TRAMPOLINE: usize = usize::MAX - PAGESIZE + 1;
/// app 用于存储 trap context 的其实地址
pub const TRAP_CONTEXT: usize = TRAMPOLINE - PAGESIZE;

/// 返回第 app_id 个 app 的OS栈顶和栈底。两个都是虚拟地址。而且每个 app 的 OS栈都不重叠
pub fn kernel_stack_position(app_id: usize) -> (usize, usize) {
    let top = TRAMPOLINE - (1 + app_id) * (KERNEL_STACK_SIZE + PAGESIZE);
    (top, top + KERNEL_STACK_SIZE)
}
