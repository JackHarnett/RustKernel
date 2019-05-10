#![no_std]
#![no_main]
#![feature(exclusive_range_pattern)]

mod colours;
mod text_buffer;

use core::panic::PanicInfo;
use colours::{Colour, ColourCode};
use crate::text_buffer::{TextWriter, ScreenBuffer};

static HELLO : &[u8] = b"Jack's Kernel!!!!";


#[panic_handler] 
fn panic_handler(_info : &PanicInfo) -> ! {
	loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {


    let mut writer = TextWriter {
        column_pos: 0,
        colour: ColourCode::new(Colour::Yellow, Colour::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut ScreenBuffer) },
    };

    writer.write_byte(b'H');
    writer.write_string("ello ");
    writer.write_string("Wörld!");

	loop {}
}