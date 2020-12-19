#![no_std]
#![no_main]
#![feature(global_asm)]
#![feature(llvm_asm)]
#![feature(panic_info_message)]
#![feature(const_in_array_repeat_expressions)]
#![feature(alloc_error_handler)]

extern crate alloc;

#[macro_use]
extern crate bitflags;

#[macro_use]
mod console;
mod lang_items;
mod sbi;
mod syscall;
mod trap;
mod config;
mod task;
mod timer;
mod mm;
mod fs;
mod drivers;

global_asm!(include_str!("entry.asm"));

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0) }
    });
}

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    println!("[kernel] Hello, world!");
    mm::init();
    mm::remap_test();
    trap::init();
    trap::enable_timer_interrupt();
    timer::set_next_trigger();
    fs::list_apps();
    //println!("after listing apps");
    task::add_initproc();
    /*
    println!("after adding initproc!");
    println!("list apps again!");
    fs::list_apps();
    println!("test user_shell now!");
    let user_shell = fs::open_file("user_shell", fs::OpenFlags::RDONLY).unwrap();
    println!("user_shell size = {}", user_shell.read_all().len());
    println!("before running tasks!");
     */
    task::run_tasks();
    panic!("Unreachable in rust_main!");
}