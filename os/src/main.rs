#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(asm_const)]
#[macro_use]
mod console;
mod lang_items;
mod sbi;

use core::arch::global_asm;

global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    clear_bss();
    println!("Hello, world!");
    print_sections();
    panic!("Shutdown machine!");
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    unsafe {
        let mut ptr = sbss as *mut u8;
        while ptr < ebss as *mut u8 {
            ptr.write_volatile(0);
            ptr = ptr.add(1);
        }
    }
}

fn print_sections() {
    extern "C" {
        fn stext();
        fn etext();
        fn srodata();
        fn erodata();
        fn sdata();
        fn edata();
        fn sbss();
        fn ebss();
        fn boot_stack();
        fn boot_stack_top();
    }
    println!(".text [{:#x}, {:#x})", stext as usize, etext as usize);
    println!(".rodata [{:#x}, {:#x})", srodata as usize, erodata as usize);
    println!(".data [{:#x}, {:#x})", sdata as usize, edata as usize);
    println!(".bss [{:#x}, {:#x})", sbss as usize, ebss as usize);
    println!("stack [{:#x}, {:#x})", boot_stack as usize, boot_stack_top as usize);
}
