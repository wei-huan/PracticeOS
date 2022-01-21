#![feature(linkage)]
#[no_mangle]
#[link_section = ".text.entry"]

pub extern "C" fn _start() -> ! {

    clear_bss();

    exit(main());

    panic!("unreachable after sys_exit!");

}

#[linkage = "weak"]
#[no_mangle]
fn main() -> i32 {

    panic!("Cannot find main!");

}