#![no_std]
#![no_main]

mod colours;

use core::panic::PanicInfo;
use colours::Colour;
// cargo xbuild --target x86_64_rustOS
// bootimage run

static HELLO : &[u8] = b"Jack's Kernel!";


#[panic_handler] 
fn panic_handler(_info : &PanicInfo) -> ! {
	loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {

	let col : colours::ColourCode = 
				colours::ColourCode::new(Colour::Red, Colour::Cyan);

	let vga_buff = 0xb8000 as *mut u8;

	for(i, &byte) in HELLO.iter().enumerate() {
		unsafe {
			*vga_buff.offset(i as isize * 2) = byte;
			*vga_buff.offset(i as isize * 2 + 1) = col.0;
		}
	}

	loop {}
}