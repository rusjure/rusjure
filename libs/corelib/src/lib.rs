extern crate core;

extern "C" {
    pub fn printf(format: *const u8, ...) -> i32;
}

#[no_mangle]
pub extern "C" fn print_hello_world() {
    unsafe {
        printf(b"Hello world!\n\0".as_ptr() as *const _);
    }
}
