//Removing Standard Library
#![no_std]

// Implementing Panic Handler
use core::panic::PanicInfo;

// when panic occurs call up this function
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}


fn main() {}
