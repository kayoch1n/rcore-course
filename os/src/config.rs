pub const USER_STACK_SIZE: usize = 4096 * 2;
pub const KERNEL_STACK_SIZE: usize = 4096 * 2;
pub const MAX_APP_NUM: usize = 128;

/// 对比：内核第一条指令的地址是 0x80200000
pub const APP_BASE_ADDRESS: usize = 0x80400000;
pub const APP_SIZE_LIMIT: usize = 0x200000;

/// QEMU 时钟频率?
pub const CLOCK_FREQ: usize = 10000000;
