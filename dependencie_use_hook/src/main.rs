fn main() {
    unsafe {
        //must add a build.rs
        //sleep 1s
        println!("{}", libc::sleep(1));
    }
}
