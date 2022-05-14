use std::os::raw::c_uint;

fn main() {
    unsafe {
        //sleep 1s
        println!("{}", libc::sleep(1));
    }
}