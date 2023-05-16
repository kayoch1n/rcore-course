pub const USER_STACK_SIZE: usize = 4096 * 2;
pub const KERNEL_STACK_SIZE: usize = 4096 * 2;
// PS: 调大这个值会有bug导致在loader::init加载第一个app的时候
// 陷入死循环。调试发现在 __all_traps 处的内容也就是代码被覆盖掉了，变成一些垃圾数据
pub const MAX_APP_NUM: usize = 64;

/// 对比：内核第一条指令的地址是 0x80200000
pub const APP_BASE_ADDRESS: usize = 0x80400000;
pub const APP_SIZE_LIMIT: usize = 0x200000;

/// QEMU 时钟频率?
pub const CLOCK_FREQ: usize = 10000000;

pub const KERNEL_HEAP_SIZE: usize = 0x30_0000;
