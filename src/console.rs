use core::fmt::{self, Write};

use crate::sbi::console_put_char;

struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.bytes() {
            console_put_char(c as usize);
        }
        Ok(())
    }
}

pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($args: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($args)+)?));
    };
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($args: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($args)+)?));
    };
}

#[macro_export]
macro_rules! error {
    ($fmt: literal $(, $($args: tt)+)?) => {
        $crate::console::print(format_args!(concat!("\x1b[31m[ERROR] ", $fmt, "\x1b[0m\n") $(, $($args)+)?));
    };
}

#[macro_export]
macro_rules! warn {
    ($fmt: literal $(, $($args: tt)+)?) => {
        $crate::console::print(format_args!(concat!("\x1b[93m[WARN] ", $fmt, "\x1b[0m\n") $(, $($args)+)?));
    };
}

#[macro_export]
macro_rules! info {
    ($fmt: literal $(, $($args: tt)+)?) => {
        $crate::console::print(format_args!(concat!("\x1b[34m[INFO] ", $fmt, "\x1b[0m\n") $(, $($args)+)?));
    };
}

#[macro_export]
macro_rules! debug {
    ($fmt: literal $(, $($args: tt)+)?) => {
        $crate::console::print(format_args!(concat!("\x1b[32m[DEBUG] ", $fmt, "\x1b[0m\n") $(, $($args)+)?));
    };
}

#[macro_export]
macro_rules! trace {
    ($fmt: literal $(, $($args: tt)+)?) => {
        $crate::console::print(format_args!(concat!("\x1b[90m[TRACE] ", $fmt, "\x1b[0m\n") $(, $($args)+)?));
    };
}
