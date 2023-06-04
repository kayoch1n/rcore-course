use core::slice::from_raw_parts;

use crate::{
    debug,
    mm::{copy_byte_buffer, translate_byte_buffer},
    task::{exit_and_run_next, suspend_and_run_next, TASK_MANAGER},
    timer,
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

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

pub fn sys_get_time(val: *mut TimeVal) -> isize {
    let token = TASK_MANAGER.get_current_token();
    let result = timer::get_time_us();
    let result = TimeVal {
        sec: result / 1_000,
        usec: result % 1_000,
    };

    let src = unsafe {
        from_raw_parts(
            &result as *const _ as *const u8,
            core::mem::size_of::<TimeVal>(),
        )
    };
    let buf = translate_byte_buffer(token, val as *mut u8, core::mem::size_of::<TimeVal>());

    copy_byte_buffer(src, buf);
    0
}
