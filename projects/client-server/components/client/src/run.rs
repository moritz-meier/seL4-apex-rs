#![no_std]

#[cfg(not(test))] //workaround
#[panic_handler]
fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}

use camkes_bindgen::client::printf;

#[no_mangle]
pub extern "C" fn run() -> isize {
    unsafe {
        printf("Starting client component ...\n\0".as_ptr());

        let a = 123;
        let b = 157;
        let res = camkes_bindgen::client::calc_add(a, b);

        printf("calc_add(%d, %d) = %d\n\0".as_ptr(), a, b, res);
    }

    0
}
