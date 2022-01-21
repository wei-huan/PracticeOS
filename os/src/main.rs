// os/src/main.rs

#![no_main]
#![no_std]
#![feature(global_asm)]
#![feature(asm)]
#![feature(panic_info_message)]

#[macro_use]
mod console;
mod lang_items;
mod sbi;

global_asm!(include_str!("entry.asm"));

fn clear_bss() {
    extern "C"{
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe{(a as *mut u8).write_volatile(0)}
    });
}

#[cfg(feature = "INFO")]
fn show_memory_layout() {
    extern "C"{
        fn BASE_ADDRESS();
    }
    (
        info!("BASE_ADDRESS: 0x{:X}", BASE_ADDRESS as usize)
    );

    extern "C"{
        fn stext();
        fn etext();
    }
    (
        info!("text: 0x{:X}, 0x{:X}", stext as usize, etext as usize)
    );

    extern "C"{
        fn sdata();
        fn edata();
    }
    (
        info!("data: 0x{:X}, 0x{:X}", sdata as usize, edata as usize)
    );

    extern "C"{
        fn srodata();
        fn erodata();
    }
    (
        info!("rodata: 0x{:X}, 0x{:X}", srodata as usize, erodata as usize)
    );

    extern "C"{
        fn sbss();
        fn ebss();
    }
    (
        info!("bss: 0x{:X}, 0x{:X}", sbss as usize, ebss as usize)
    );
}


#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();

    #[cfg(feature = "INFO")]
    show_memory_layout();

    info!("Hello, world!");
    warn!("Hello, world!");
    error!("Hello, world!");
    panic!("Shutdown machine!");
}

