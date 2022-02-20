use crate::mm::{translated_byte_buffer, alloc_pages};
use crate::task::current_user_token;

const FD_STDOUT: usize = 1;

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    match fd {
        FD_STDOUT => {
            let buffers = translated_byte_buffer(current_user_token(), buf, len);
            for buffer in buffers {
                print!("{}", core::str::from_utf8(buffer).unwrap());
            }
            len as isize
        }
        _ => {
            panic!("Unsupported fd in sys_write!");
        }
    }
}


// syscall ID：222
// 申请长度为 len 字节的物理内存（不要求实际物理内存位置，可以随便找一块），将其映射到 start 开始的虚存，内存页属性为 port
// 参数：start 需要映射的虚存起始地址，要求按页对齐
//      len 映射字节长度，可以为 0
//      port：第 0 位表示是否可读，第 1 位表示是否可写，第 2 位表示是否可执行。其他位无效且必须为 0
// 返回值：执行成功则返回 0，错误返回 -1
pub fn sys_mmap(start: usize, len: usize, port: usize) -> isize {
    if (start & 0xFFF) > 0 || (port & !0x7) > 0 || (port & 0x7) == 0{
        return -1;
    }

    alloc_pages(current_user_token(), start, len, port)
}

// syscall ID：215
// 取消到 [start, start + len) 虚存的映射
// 参数：start 需要映射的虚存起始地址，要求按页对齐
//      len 映射字节长度，可以为 0
// 返回值：执行成功则返回 0，错误返回 -1
// pub fn sys_munmap(start: usize, len: usize) -> isize {

// }
