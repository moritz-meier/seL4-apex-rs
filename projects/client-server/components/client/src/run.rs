#![no_std]

#[cfg(not(test))] //workaround
#[panic_handler]
fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}

#[no_mangle]
extern "C" fn rust_eh_personality() {}

use camkes_bindgen::client::{calc_add, calc_div, calc_mul, calc_sub, get_instance_name, printf};

#[no_mangle]
pub extern "C" fn run() -> isize {
    unsafe {
        printf("Starting %s ...\n\0".as_ptr(), get_instance_name());

        let a = 123;
        let b = 157;
        let res = calc_add(a, b);

        printf("calc_add(%d, %d) = %d\n\0".as_ptr(), a, b, res);

        let a = 133;
        let b = 17;
        let res = calc_sub(a, b);

        printf("calc_sub(%d, %d) = %d\n\0".as_ptr(), a, b, res);

        let a = 78;
        let b = 138;
        let res = calc_mul(a, b);

        printf("calc_mul(%d, %d) = %d\n\0".as_ptr(), a, b, res);

        let a = 123;
        let b = 38;
        let res = calc_div(a, b);

        printf("calc_div(%d, %d) = %d\n\0".as_ptr(), a, b, res);
    }

    0
}
