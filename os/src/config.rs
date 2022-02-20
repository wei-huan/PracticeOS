// 用户栈 8KiB
pub const USER_STACK_SIZE: usize = 4096 * 2;

// 内核栈 8KiB
pub const KERNEL_STACK_SIZE: usize = 4096 * 2;

// 3MiB 的内核堆区
pub const KERNEL_HEAP_SIZE: usize = 0x30_0000;

// k210 Flash内存一共 8MiB, 从 0x80000000 到 0x80800000
pub const MEMORY_END: usize = 0x80800000;

// 一页 4KiB
pub const PAGE_SIZE: usize = 0x1000;

// 页内地址占据 12bit 位
pub const PAGE_SIZE_BITS: usize = 0xc;

// TRAMPOLINE 的地址
pub const TRAMPOLINE: usize = usize::MAX - PAGE_SIZE + 1;

// TRAP_CONTEXT 的地址
pub const TRAP_CONTEXT: usize = TRAMPOLINE - PAGE_SIZE;

// 返回应用程序的内核地址空间的内核栈位置
/// Return (bottom, top) of a kernel stack in kernel space.
pub fn kernel_stack_position(app_id: usize) -> (usize, usize) {
    let top = TRAMPOLINE - app_id * (KERNEL_STACK_SIZE + PAGE_SIZE);
    let bottom = top - KERNEL_STACK_SIZE;
    (bottom, top)
}

pub use crate::board::CLOCK_FREQ;
