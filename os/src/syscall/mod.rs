use self::{fs::sys_write, proc::sys_exit};

mod fs;
mod proc;

const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;

pub fn syscall(id: usize, args: [usize; 3]) -> isize {
    match id {
        SYSCALL_WRITE => sys_write(args[0], args[1] as *const u8, args[2]),
        SYSCALL_EXIT => sys_exit(args[0] as isize),
        _ => panic!("syscall {} is not supported", id),
    }
}
