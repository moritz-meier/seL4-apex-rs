#![no_std]

#[cfg(not(test))] //workaround
#[panic_handler]
fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}

use camkes_bindgen::server::printf;

#[no_mangle]
pub extern "C" fn run() -> isize {
    unsafe {
        printf("Starting server component ...\n\0".as_ptr());
    }

    0
}
