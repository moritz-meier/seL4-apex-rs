#![no_std]

#[cfg(not(test))] //workaround
#[panic_handler]
fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}

use camkes_bindgen::main_obj;

extern "C" {
    fn seL4_DebugDumpScheduler();
}

#[no_mangle]
pub extern "C" fn run() -> isize {
    unsafe {
        seL4_DebugDumpScheduler();
    }

    0
}
