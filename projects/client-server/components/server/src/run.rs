#![no_std]

#[cfg(not(test))] //workaround
#[panic_handler]
fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}

#[no_mangle]
extern "C" fn rust_eh_personality() {}

use camkes_bindgen::server::{get_instance_name, printf};

#[no_mangle]
pub extern "C" fn run() -> isize {
    unsafe {
        printf("Starting %s ...\n\0".as_ptr(), get_instance_name());
    }

    0
}

#[no_mangle]
pub extern "C" fn calc_add(a: isize, b: isize) -> isize {
    a + b
}

#[no_mangle]
pub extern "C" fn calc_sub(a: isize, b: isize) -> isize {
    a - b
}

#[no_mangle]
pub extern "C" fn calc_mul(a: isize, b: isize) -> isize {
    a * b
}

#[no_mangle]
pub extern "C" fn calc_div(a: isize, b: isize) -> isize {
    a / b
}
