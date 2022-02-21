#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

extern crate alloc;

#[macro_use]
extern crate bitflags;

#[cfg(feature = "board_k210")]
#[path = "boards/k210.rs"]
mod board;
#[cfg(not(any(feature = "board_k210")))]
#[path = "boards/qemu.rs"]
mod board;

#[macro_use]
mod console;
mod config;
mod lang_items;
mod loader;
mod mm;
mod sbi;
mod sync;
mod syscall;
mod task;
mod timer;
mod trap;

use core::arch::global_asm;

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    unsafe {
        core::slice::from_raw_parts_mut(sbss as usize as *mut u8, ebss as usize - sbss as usize)
            .fill(0);
    }
}

extern "C" {
    fn stext();
    fn etext();
    fn srodata();
    fn erodata();
    fn sdata();
    fn edata();
    fn sbss_with_stack();
    fn sbss();
    fn ebss();
    fn ekernel();
    fn strampoline();
}

#[allow(unused)]
fn show_layout(){
    let stext_clone = (stext as usize).clone();
    let etext_clone = (etext as usize).clone();

    println!("text({}MiB): 0x{:X} - 0x{:X}", (((etext_clone - stext_clone) as f32) / ((1024 * 1024) as f32)), stext as usize, etext as usize);
    println!("strampoline: 0x{:X}", strampoline as usize);
    println!("rodata: 0x{:X} - 0x{:X}", srodata as usize, erodata as usize);
    println!("data: 0x{:X} - 0x{:X}", sdata as usize, edata as usize);
    println!("bss: 0x{:X} - 0x{:X}", sbss_with_stack as usize, ebss as usize);
    println!("sbss: 0x{:X}", sbss as usize);
}

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    println!("[kernel] Hello, world!");
    mm::init();
    mm::remap_test();
    task::add_initproc();
    println!("after initproc!");
    trap::init();
    trap::enable_timer_interrupt();
    timer::set_next_trigger();
    loader::list_apps();
    task::run_tasks();
    panic!("Unreachable in rust_main!");
}

// #[no_mangle]
// pub fn rust_main() -> ! {
//     show_layout();
//     clear_bss();
//     println!("[kernel] Hello, world!");
//     mm::heap_allocator::init_heap();
//     mm::frame_allocator::init_frame_allocator();
//     mm::heap_allocator::heap_test();
//     mm::frame_allocator::frame_allocator_test();
//     panic!("Unreachable in rust_main!");
// }
