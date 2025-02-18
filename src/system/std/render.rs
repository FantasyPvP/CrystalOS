use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use crate::system::kernel::render::{RENDERER, ScreenChar};
use crate::std::io::Color;
use num_traits::{Num, ToPrimitive};

/// TODO: get a working implementation for CLI apps
/// elements can be created using their from_str() method
/// you can then render the element to the current frame using the render() method
/// the position of the element by passing a tuple (x,y) to render()
///
/// nothing will appear on the screen until the frame is actually rendered by
/// the write_to_screen() method on the renderer

pub use crate::system::kernel::render::{
    special_char,
    RenderError,
    ColorCode,
    BUFFER_WIDTH,
    BUFFER_HEIGHT
};

pub struct Window {
    width: u32,
    height: u32,
    x: u32,
    y: u32,
    bordered: bool,
    open: bool,
}

impl Window {
    pub fn new() -> Window {
        Window {
            width: BUFFER_WIDTH as u32,
            height: BUFFER_HEIGHT as u32,
            x: 0,
            y: 0,
            bordered: true,
            open: true
        }
    }

    pub fn open(&mut self) -> Result<(), RenderError> {
        if self.open {
            return Err(RenderError::InvalidRenderMode);
        }
        self.open = true;

        let mut frame: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT] = [[ScreenChar::null(); BUFFER_WIDTH]; BUFFER_HEIGHT];


        Ok(())
    }

    // pub fn render(&self, f: &Frame) -> Result<(), RenderError> {
    //     let mut frame: &[[ScreenChar; self.width]; self.height] = &[[ScreenChar::null(); self.width]; self.height];
        
        
        
    //     for (i, row) in f.frame.iter().enumerate() {
    //         for (j, col) in row.iter().enumerate() {
    //             frame[i + f.position.y][j + f.position.x] = col.as_screen_char();
    //         };
    //     }
    //     RENDERER.lock().render_frame(frame);
    //     Ok(())
    // }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColouredChar {
    pub character: char,
    pub colour: ColorCode,
}

impl ColouredChar {
    pub fn coloured(character: char, colour: ColorCode) -> ColouredChar {
        ColouredChar { character, colour }
    }
    pub fn new(character: char) -> ColouredChar {
        ColouredChar {
            character,
            colour: ColorCode::new(Color::White, Color::Black),
        }
    }
    pub fn null() -> ColouredChar {
        ColouredChar {
            character: ' ',
            colour: ColorCode::new(Color::White, Color::Black),
        }
    }
    pub fn as_screen_char(&self) -> ScreenChar {
        ScreenChar {
            character: {
                if let Some(c) = special_char(self.character) {
                    c
                } else {
                    self.character as u8
                }
            },
            colour: self.colour,
        }
    }
}




#[derive(Copy, Clone, Debug)]
pub struct Position<T: Num> {
    pub x: T,
    pub y: T,
}

impl<T: Num + ToPrimitive> Position<T> {
    pub fn new(x: T, y: T) -> Position<T> {
        Position { x, y }
    }

    pub fn zero() -> Position<T> {
        Position { x: T::zero(), y: T::zero() }
    }
    
    pub fn into_usize(self) -> Result<Position<usize>, ()> { 
        Ok(Position { 
            x: self.x.to_usize().ok_or(())?, 
            y: self.y.to_usize().ok_or(())?,
        })
    }
    pub fn into_i32(self) -> Result<Position<i32>, ()> {
        Ok(Position {
            x: self.x.to_i32().ok_or(())?,
            y: self.y.to_i32().ok_or(())?,
        })
    }

    pub fn into_f32(self) -> Result<Position<f32>, ()> {
        Ok(Position {
            x: self.x.to_f32().ok_or(())?,
            y: self.y.to_f32().ok_or(())?,
        })
    }
}

pub type Dimensions<T> = Position<T>;


#[derive(Clone, Debug)]
pub struct Frame {
    pub position: Position<usize>,
    pub dimensions: Dimensions<usize>,
    pub frame: Vec<Vec<ColouredChar>>,
}


impl Frame {
    pub fn new(position: Position<usize>, dimensions: Dimensions<usize>) -> Result<Frame, RenderError> {
        Ok(Frame {
            position,
            dimensions,
            frame: vec![vec![ColouredChar::null(); dimensions.x]; dimensions.y],
        })
    }

    pub fn from_window(window: &Window) -> Frame {
        Frame {
            dimensions: Dimensions::new(window.width as usize, window.height as usize),
            position: Position::new(window.x as usize, window.y as usize),
            frame: vec![vec![ColouredChar::null(); window.width as usize]; window.height as usize],
        }
    }

    pub fn from_str(elemstr: String) -> Self {
        let mut element = Frame { frame: Vec::<Vec<ColouredChar>>::new(), dimensions: Dimensions::new(0, 0), position: Position::new(0, 0) };

        for line in elemstr.split("\n") {
            element.frame.push(
                line.chars()
                    .map(|c| ColouredChar::new(c))
                    .collect::<Vec<ColouredChar>>()
            );
        }

        for row in element.clone().frame {
            let n = row.len();
            if n > element.dimensions.x {
                element.dimensions.x = n;
            }
        }
        element
    }

    pub fn render(&self) -> Vec<Vec<ColouredChar>> {
        self.frame.clone()
    }

    /// DEPRECATED. Use Window::render(&Frame) instead !!
    pub fn write_to_screen(&self) -> Result<(), RenderError> {
        let mut frame: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT] = [[ScreenChar::null(); BUFFER_WIDTH]; BUFFER_HEIGHT];
        for (i, row) in self.frame.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                frame[i + self.position.y][j + self.position.x] = col.as_screen_char();
            };
        }
        RENDERER.lock().render_frame(frame);
        Ok(())
    }

    pub fn get_position(&self) -> Position<usize> {
        self.position
    }
    pub fn set_position(&mut self, position: Position<usize>) {
        self.position = position
    }
    pub fn dimensions(&self) -> Dimensions<usize> {
        self.dimensions
    }
    pub fn write(&mut self, position: Position<usize>, char: ColouredChar) -> Result<(), RenderError> {
        if position.x >= self.dimensions.x || position.y >= self.dimensions.y {
            return Err(RenderError::OutOfBounds(
                position.x >= self.dimensions.x,
                position.y >= self.dimensions.y,
            ));
        }
        self.frame[position.y][position.x] = char;
        Ok(())
    }
    pub fn place_child_element(&mut self, other: &Frame) {
        for (i, row) in other.frame.iter().enumerate() {
            for (j, chr) in row.iter().enumerate() {
                self.frame[i + other.position.y][j + other.position.x] = *chr
            }
        }
    }

    pub fn render_bounds_check(&self, element: &Frame, should_panic: bool) -> Result<(), RenderError> {

        let (mut x, mut y) = (false, false);

        if element.dimensions().x + element.get_position().x > self.dimensions.x {
            if should_panic { panic!(
                "Element is to large to be rendered {} {}",
                element.dimensions().x + element.get_position().x,
                self.dimensions.x
            )} else {
                x = true;
            }
        }

        if element.dimensions().y + element.get_position().y > self.dimensions.y {
            if should_panic { panic!(
                "Element is to large to be rendered {} {}",
                element.dimensions().y + element.get_position().y,
                self.dimensions.y
            )} else {
                y = true;
            }
        }

        if x || y {
            Err(RenderError::OutOfBounds(x, y))
        } else {
            Ok(())
        }
    }
}

impl core::ops::Index<usize> for Frame {
    type Output = Vec<ColouredChar>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.frame[index]
    }
}

impl core::ops::IndexMut<usize> for Frame {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.frame[index]
    }
}
































