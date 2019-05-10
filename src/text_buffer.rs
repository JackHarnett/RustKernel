use crate::colours::ColourCode;

use volatile::Volatile;
// Number of characters per line
const BUFFER_WIDTH : usize = 80;

// Number of lines to be displayed
const BUFFER_HEIGHT	: usize = 25;

/* 
 * Represenets a coloured ascii character.
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
	ascii_character: u8,
	colour : ColourCode
}

/**
 * Represents the entire screen as an array of [ScreenChar].
 */
#[repr(transparent)]
pub struct ScreenBuffer { 
	chars : [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

/**
 * Allows text to be written to the VGA Buffer.
 */
pub struct TextWriter {
	pub column_pos : usize,
	pub colour : ColourCode,
	pub buffer : &'static mut ScreenBuffer,
}


impl TextWriter {

	pub fn write_byte(&mut self, byte : u8) {
		match byte {
			b'\n' => self.new_line(),

			byte  => {
				if self.column_pos >= BUFFER_WIDTH {
					self.new_line();
				}

				let row = BUFFER_HEIGHT-1;
				let col = self.column_pos;

				self.buffer.chars[row][col].write(ScreenChar {
					ascii_character : byte,
					colour : self.colour,
				});

				self.column_pos += 1;
			}
		}
	}

	pub fn write_string(&mut self, string : &str) {
		for byte in string.bytes() {
			match byte {
				0x20..0x7e | b'\n' => self.write_byte(byte),
				_ => self.write_byte(0xfe),
			}
		}
	}

	pub fn new_line(&mut self) {

	}
}

