#![no_std]

use core::ffi::CStr;

extern {
    pub fn printf(format: *const u8, ...) -> i32;
}

#[no_mangle]
pub extern "C" fn addition(a: usize, b: usize) -> usize {
    a + b
}

#[no_mangle]
pub unsafe extern "C" fn println(str: *const i8) {
    unsafe {
        let str = CStr::from_ptr(str);
        let str = str.to_str().unwrap();
        printf(b"%s\n" as *const u8, str);
    }
}
