#![no_std]
#![no_main]
//Removing Standard Library and main function
// Implementing Panic Handler
use core::panic::PanicInfo;

//Making a new entry point
static HELLO: &[u8] = b"Hello World!";//global variable called HELLO making an array of 8-bit integers

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;// vga buffer address while making it a mutable 8-bit integer
//for i (=chars in HELLO) and 
    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;//writing bytes to vga buffer for i
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
    loop {}
}/*Using C calling convention. Also labled
_start as the entry point. Made it diverging with !
so there is no return */

// when panic occurs call up this function
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

