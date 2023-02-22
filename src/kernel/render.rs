
use volatile::Volatile;
use lazy_static::lazy_static;
use core::fmt;
use spin::Mutex;

use alloc::vec::Vec;
use alloc::vec;
use alloc::borrow::ToOwned;

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

pub const BUFFER_HEIGHT: usize = 25;
pub const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
	chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

struct BufferSwap {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}
struct CharGrid {
    chars: Vec<[ScreenChar; BUFFER_WIDTH]>
}

pub struct Renderer {
	col_pos: usize,
	col_code: ColorCode,
	buffer: &'static mut Buffer,
    userspace: BufferSwap,
    upwards: CharGrid,
    downwards: CharGrid,
    pub sandbox: bool,
}


lazy_static! {
	pub static ref RENDERER: Mutex<Renderer> = Mutex::new(Renderer {
		col_pos: 0,
		col_code: ColorCode::new(Color::White, Color::Black),
		buffer: unsafe {
			&mut *(0xb8000 as *mut Buffer)
		},
        userspace: BufferSwap {
            chars: [[ScreenChar {
                character: 179u8,
                colour: ColorCode::new(Color::White, Color::Black),
            }; BUFFER_WIDTH]; BUFFER_HEIGHT]
        },
        upwards: CharGrid {
            chars: vec![
                [ScreenChar {
                    character: 32u8,
                    colour: ColorCode::new(Color::White, Color::Black),
                }; 80]
            ]
        },   
        downwards: CharGrid {
            chars: vec![
                [ScreenChar {
                    character: 32u8,
                    colour: ColorCode::new(Color::White, Color::Black),
                }; 80]
            ]
        },
        sandbox: false,
	});
}



impl Renderer {

    pub fn text_mode(&mut self) -> Result<(), ()> { 
        if !self.sandbox { return Err(()) };
        self.buffer_swap().unwrap();
        self.sandbox = false;
        Ok(())
    }

    pub fn sandbox_mode(&mut self) -> Result<(), ()> { 
        if self.sandbox { return Err(()) };
        self.buffer_swap().unwrap();
        self.sandbox = true;
        Ok(())
    }

    fn buffer_swap(&mut self) -> Result<(), ()> {

        for (i, _) in self.userspace.chars.clone().iter().enumerate() {

            let tmp = self.buffer.chars[i].clone();

            for (j, col) in self.userspace.chars[i].clone().iter().enumerate() {
                self.buffer.chars[i][j].write(col.to_owned())
            }

            for (j, _) in tmp.iter().enumerate() {
                self.userspace.chars[i][j] = tmp[j].read().to_owned()
            }
        }

        Ok(())
    }

	pub fn render_frame(&mut self, frame: [ [ char; BUFFER_WIDTH ]; BUFFER_HEIGHT]) {
		for (i, row) in frame.iter().enumerate() {
			for (j, col) in row.iter().enumerate() {

				if let Some(c) = self.fancy_char(*col) {
					self.buffer.chars[i][j].write(ScreenChar { character: c, colour: self.col_code});
				} else {
					self.buffer.chars[i][j].write(ScreenChar { character: *col as u8, colour: self.col_code});
				}
			}
		}
	}


	pub fn write_string(&mut self, string: &str) {
		for ch in string.chars() {

			if let Some(x) = self.fancy_char(ch) {
				self.write_byte(x)
			} else {
				match ch as u8 {
					0x20..=0xff | b'\n' => self.write_byte(ch as u8),
					_ => self.write_byte(0xfe),
				}	
			}	
		}
	}

	fn fancy_char(&self, ch: char) -> Option<u8> {
		let res: u8 = match ch {
			'│' => 179,
			'─' => 196,
			'┴' => 193,
			'┤'	=> 180,
			'═' => 205,
			'║'	=> 186,
			'╗' => 187,
			'╝' => 188,
			'╚' => 200,
			'╔' => 201,
			'»' => 175,
			'┐' => 191,
			'└' => 192,
			'┘' => 217,
			'┌' => 218,		
			_ => { return None; }
		};
		Some(res)
	}

	pub fn backspace(&mut self) -> Result<(), ()> {
		if self.col_pos == 0 {
			self.undonewline();
		}	
		self.col_pos -= 1;
		let row = BUFFER_HEIGHT -1;
		let col = self.col_pos;

		let blank = ScreenChar {
			character: b' ',
			colour: self.col_code,
		};
		self.buffer.chars[row][col].write(blank);		
		Ok(())
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
	
	pub fn undonewline(&mut self) {
		for row in (0..BUFFER_HEIGHT-1).rev() {
			for col in 0..BUFFER_WIDTH {
				let character = self.buffer.chars[row][col].read();
					self.buffer.chars[row + 1][col].write(character);
			}
		}
		self.clear_row(0);
		self.col_pos = BUFFER_WIDTH;
	}
	pub fn clear(&mut self) {
		for row in (0..BUFFER_HEIGHT-1).rev() {
			self.clear_row(row);
		}
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

impl fmt::Write for Renderer {
	fn write_str(&mut self, string:&str) -> fmt::Result {
		self.write_string(string);
		Ok(())
	}
}

pub fn write(args: fmt::Arguments, cols: (Color, Color)) {
	use core::fmt::Write;
	use x86_64::instructions::interrupts;
	interrupts::without_interrupts(|| {
		let mut writer = RENDERER.lock();
		writer.col_code = ColorCode::new(cols.0, cols.1);
		writer.write_fmt(args).unwrap()
	})
}

#[doc(hidden)]
pub fn _log(args: fmt::Arguments) {
	use core::fmt::Write;
	use x86_64::instructions::interrupts;

	interrupts::without_interrupts(|| {
		let mut writer = RENDERER.lock();
		writer.col_code = ColorCode::new(Color::Yellow, Color::Black);
		writer.write_fmt(args).unwrap();
		
		//WRITER.lock().write_fmt(args).unwrap();
	});
}

#[macro_export]
macro_rules! println_log {
	() => ($crate::print_log!("/n"));
	($($arg:tt)*) => ($crate::print_log!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! print_log {
	($($arg:tt)*) => ($crate::render::_log(format_args!($($arg)*)));
}


#[macro_export]
macro_rules! println {
	() => ($crate::print!("/n"));
	($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! print {
	($($arg:tt)*) => ($crate::kernel::render::_print(format_args!($($arg)*)));
}




#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
	use core::fmt::Write;
	use x86_64::instructions::interrupts;

	interrupts::without_interrupts(|| {
		let mut writer = RENDERER.lock();
		writer.col_code = ColorCode::new(Color::White, Color::Black);
		writer.write_fmt(args).unwrap();
		
		//WRITER.lock().write_fmt(args).unwrap();
	});
}
