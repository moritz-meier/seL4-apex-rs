#![no_std]

#[cfg(not(test))] //workaround
#[panic_handler]
fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}

#[no_mangle]
extern "C" fn rust_eh_personality() {}

use camkes_bindgen::server::printf;

#[no_mangle]
pub extern "C" fn run() -> isize {
    unsafe {
        printf("Starting server component ...\n\0".as_ptr());
    }

    0
}

#[no_mangle]
pub extern "C" fn calc_add(a: isize, b: isize) -> isize {
    a + b
}
