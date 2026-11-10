//Removing Standard Library and main function
#![no_std]
#![no_main]
// Implementing Panic Handler
use core::panic::PanicInfo;

// when panic occurs call up this function
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

fn main() {}
