#![no_std]

#[cfg(not(test))] //workaround
#[panic_handler]
fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}

use camkes_bindgen::printf;

#[no_mangle]
pub extern "C" fn run() -> isize {
    unsafe {
        printf("Hello Camkes, Rust!\n\0".as_ptr());
    }

    0
}
