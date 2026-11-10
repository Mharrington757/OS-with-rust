//Removing Standard Library and main function
#![no_std]
#![no_main]
// Implementing Panic Handler
use core::panic::PanicInfo;

//Making a new entry point
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    loop {}
}/*Using C calling convention. Also labled
_start as the entry point. Made it diverging with !
so there is no return */

// when panic occurs call up this function
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

fn main() {}
