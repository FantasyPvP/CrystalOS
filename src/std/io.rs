use crate::{
    kernel::render::{Color, write, RENDERER, BUFFER_WIDTH, BUFFER_HEIGHT},
    kernel::tasks::keyboard::KEYBOARD,
    kernel::os::OS,
};
use alloc::{boxed::Box, string::{String, ToString}, vec::Vec};

pub use crate::{print, println, serial_print, serial_println};

use lazy_static::lazy_static;
use spin::Mutex;

pub async fn stdin() -> String {
    let string = KEYBOARD.lock().get_string().await;
    string
}

pub async fn stdchar() -> char {
    let chr = KEYBOARD.lock().get_keystroke().await;
    chr
}


pub type Frame = [ [ char; BUFFER_WIDTH ]; BUFFER_HEIGHT];

#[derive(Clone)]
pub struct Element {
    frame: Vec<Vec<char>>,
    dimensions: (u8, u8)
}
/// elements can be created using their from_str() method
/// you can then render the element to the current frame using the render() method
/// the position of the element by passing a tuple (x,y) to render()
/// 
/// nothing will appear on the screen until the frame is actually
impl Element {
    pub fn from_str(elemstr: String) -> Self {
        let mut element = Element { frame: Vec::<Vec<char>>::new(), dimensions: (0, 0) }; 

        for line in elemstr.split("\n") {
            let mut ln = Vec::<char>::new();
            for col in line.chars() {
                ln.push(col)
            };
            element.frame.push(ln);
        }

        for row in element.clone().frame {
            let n = row.len();
            if n > element.dimensions.0 as usize {
                element.dimensions.0 = n as u8;
            }
        }
        element
    }

    pub fn render(&mut self,  pos: (u8, u8)) { // x,y
        for (i, row) in self.frame.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                println!("{} {} {}", i, j, col);
                FRAMEGEN.lock().frame[i + pos.1 as usize][j + pos.0 as usize] = *col;
            };
        }
    }
}


lazy_static! {
    pub static ref FRAMEGEN: Mutex<FrameGen> = Mutex::new(FrameGen::new() );
}


#[derive(Clone, Copy)]
pub struct FrameGen {
    frame: Frame,
}


impl FrameGen {
    pub fn render_frame(&self) {
        RENDERER.lock().render_frame(self.frame)
    }

    fn new() -> Self {
        let mut frame: [[char; BUFFER_WIDTH]; BUFFER_HEIGHT] = [[' '; BUFFER_WIDTH]; BUFFER_HEIGHT];
        for i in 0..BUFFER_WIDTH {
            frame[0][i] = "┌──────────────────────────────────────────────────────────────────────────────┐".chars().collect::<Vec<char>>()[i];
            frame[BUFFER_HEIGHT -1][i] = "└──────────────────────────────────────────────────────────────────────────────┘".chars().collect::<Vec<char>>()[i];
        }
        
        for j in 1..BUFFER_HEIGHT -1 {
            for i in 0..BUFFER_WIDTH {
                frame[j][i] = "│                                                                              │".chars().collect::<Vec<char>>()[i];               
            }
        }

        Self { frame: Frame::from(frame) }
    }

    pub fn get_frame(&self) -> &[ [ char; BUFFER_WIDTH ]; BUFFER_HEIGHT] {
        &self.frame
    }

}


impl core::fmt::Display for FrameGen {
    fn fmt(&self, _: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        println!(" ");
        for row in &self.frame {
            println!("{}", row.iter().collect::<String>());
        };
        Ok(())
    }
}

