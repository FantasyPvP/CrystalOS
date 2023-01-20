
use volatile::Volatile;
use lazy_static::lazy_static;
use core::fmt;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
	fn new(foreground: Color, background: Color) -> ColorCode {
		ColorCode((background as u8) << 5 | (foreground as u8))
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
	character: u8,
	colour: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
	chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
	col_pos: usize,
	col_code: ColorCode,
	buffer: &'static mut Buffer,
}

impl Writer {
	pub fn write_string(&mut self, string: &str) {
		for byte in string.bytes() {
			match byte {
				0x20..=0x7e | b'\n' => self.write_byte(byte),
				_ => self.write_byte(0xfe),
			}
		}
	}
	pub fn write_byte(&mut self, byte: u8) {
		match byte {
			b'\n' => {
				self.newline()
			},
			byte => {
				if self.col_pos >= BUFFER_WIDTH {
					self.newline();
				}
				let row = BUFFER_HEIGHT -1;
				let col = self.col_pos;
				let col_code = self.col_code;
				self.buffer.chars[row][col].write(ScreenChar {
					character: byte,
					colour: col_code,
				});
				self.col_pos += 1
			}
		}
	}
	fn newline(&mut self) {
		for row in 1..BUFFER_HEIGHT {
			for col in 0..BUFFER_WIDTH {
				let character = self.buffer.chars[row][col].read();
				self.buffer.chars[row - 1][col].write(character);
			}
		}
		self.clear_row(BUFFER_HEIGHT -1);
		self.col_pos = 0;
	}
	
    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            character: b' ',
            colour: self.col_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }
}

impl fmt::Write for Writer {
	fn write_str(&mut self, string:&str) -> fmt::Result {
		self.write_string(string);
		Ok(())
	}
}


use spin::Mutex;

lazy_static! {
	pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
		col_pos: 0,
		col_code: ColorCode::new(Color::Yellow, Color::Black),
		buffer: unsafe {
			&mut *(0xb8000 as *mut Buffer)
		},
	});
}

#[macro_export]
macro_rules! println {
	() => ($crate::print!("/n"));
	($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! print {
	($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
	use core::fmt::Write;
	use x86_64::instructions::interrupts;

	interrupts::without_interrupts(|| {
		WRITER.lock().write_fmt(args).unwrap();
	})

}

#[test_case]
fn check_println_out() {

	use core::fmt::Write;
	use x86_64::instructions::interrupts;

	let string = "a string to be printed or something";

	interrupts::without_interrupts(|| {
		let mut writer = WRITER.lock();
		writeln!(writer, "\n{}", string).expect("failed to write string");
    	for (i, c) in string.chars().enumerate() {
	        let screen_char = writer.buffer.chars[BUFFER_HEIGHT - 2][i].read();
	        assert_eq!(char::from(screen_char.character), c);
    	}
	});


}
