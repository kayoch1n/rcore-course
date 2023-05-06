use crate::{
    debug,
    task::{exit_and_run_next, suspend_and_run_next},
};

/// 退出当前app，执行下一个app
pub fn sys_exit(code: isize) -> ! {
    debug!("Application exited with code {}", code);
    exit_and_run_next();
    panic!("unreachable");
}

/// 挂起当前app，执行下一个app
pub fn sys_yield() -> isize {
    debug!("Application suspended");
    suspend_and_run_next();
    0
}
