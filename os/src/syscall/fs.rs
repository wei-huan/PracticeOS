use crate::batch::get_cur_app_lowlimit;
use crate::batch::get_cur_app_uplimit;

const FD_STDOUT: usize = 1;

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    // println!("addr: {:?}", buf);

    if (buf as usize) >= get_cur_app_uplimit() || (buf as usize) < get_cur_app_lowlimit() {
        panic!("illegal Address write");
    }

    match fd {
        FD_STDOUT => {
            let slice = unsafe { core::slice::from_raw_parts(buf, len) };
            let str = core::str::from_utf8(slice).unwrap();
            print!("{}", str);
            len as isize
        },
        _ => {
            panic!("Unsupported fd in sys_write!");
        }
    }
}