#![no_std]
#![no_main]

use core::arch::asm;
use core::fmt::{self, Write};
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// 系统调用封装
const SYSCALL_EXIT: usize = 93;
const SYSCALL_WRITE: usize = 64;

fn syscall(id: usize, args: [usize; 3]) -> isize {
    let mut ret: isize;
    unsafe {
        asm!(
            "ecall",
            in("x10") args[0],
            in("x11") args[1],
            in("x12") args[2],
            in("x17") id,
            lateout("x10") ret,
        );
    }
    ret
}

pub fn sys_exit(code: i32) -> isize {
    syscall(SYSCALL_EXIT, [code as usize, 0, 0])
}

pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
    syscall(SYSCALL_WRITE, [fd, buffer.as_ptr() as usize, buffer.len()])
}

// 实现打印
struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        sys_write(1, s.as_bytes());
        Ok(())
    }
}

pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($fmt:literal $(, $($arg:tt)+)?) => {
        $crate::print(format_args!($fmt $(, $($arg)+)?));
    };
}

#[macro_export]
macro_rules! println {
    ($fmt:literal $(, $($arg:tt)+)?) => {
        $crate::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    };
}

// 程序入口
#[no_mangle]
extern "C" fn _start() {
    println!("Hello, world!");
    sys_exit(0);
}
