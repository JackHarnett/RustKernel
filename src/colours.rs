
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Colour {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}


// Represents a colour as 8 bits. 
#[repr(transparent)]
pub struct ColourCode(pub u8);

impl ColourCode {

	pub fn new(fg : Colour, bg : Colour) -> ColourCode {
		ColourCode ( ( (bg as u8) << 4 | (fg as u8)) )
	}
}