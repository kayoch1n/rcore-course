use self::{
    fs::sys_write,
    proc::{sys_exit, sys_get_time, sys_yield, TimeVal},
};

pub mod fs;
pub mod proc;

const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_YIELD: usize = 124;
const SYSCALL_GETTIMEOFDAY: usize = 169;

pub fn syscall(id: usize, args: [usize; 3]) -> isize {
    match id {
        SYSCALL_WRITE => sys_write(args[0], args[1] as *const u8, args[2]),
        SYSCALL_EXIT => sys_exit(args[0] as isize),
        SYSCALL_YIELD => sys_yield(),
        SYSCALL_GETTIMEOFDAY => sys_get_time(args[0] as *mut TimeVal),
        _ => panic!("syscall {} is not supported", id),
    }
}
