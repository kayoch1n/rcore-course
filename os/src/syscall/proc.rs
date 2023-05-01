use crate::{
    debug,
    task::{exit_and_run_next, suspend_and_run_next},
};

pub fn sys_exit(code: isize) -> ! {
    debug!("Application exited with code {}", code);
    exit_and_run_next();
    panic!("unreachable");
}

pub fn sys_yield() -> isize {
    debug!("Application suspended");
    suspend_and_run_next();
    0
}
