// https://os.phil-opp.com/freestanding-rust-binary/
//
// To build for a bare metal ARM target:
//
//    rustup target add thumbv7em-none-eabihf
//    cargo build --target thumbv7em-none-eabihf
//

#![no_std]
#![no_main]

use core::panic::PanicInfo;

/// This function is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
