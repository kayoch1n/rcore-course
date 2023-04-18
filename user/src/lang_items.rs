use core::panic::PanicInfo;

use crate::println;

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!("panicked at {}:{} {}", location.file(), location.line(), info.message().unwrap())
    } else {
        println!("panicked at {}", info.message().unwrap())
    }
    loop { }
}
