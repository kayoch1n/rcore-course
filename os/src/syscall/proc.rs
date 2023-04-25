use crate::{batch::run_next_app, debug};

pub fn sys_exit(code: isize) -> ! {
    debug!("Application exited with code {}", code);
    run_next_app()
}
