#![no_std]

#[cfg(not(test))] //workaround
#[panic_handler]
fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}

#[allow(dead_code)]
extern "C" {
    fn printf(val: *const u8);
}

#[no_mangle]
pub extern "C" fn run() -> isize {
    unsafe { printf("Hello Rust-World!\n\0".as_ptr()) }

    0
}
