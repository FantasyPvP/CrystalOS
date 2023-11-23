use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use core::slice::from_mut;
use crate::kernel::render::{ColorCode, RenderError, ScreenChar};
use crate::{printerr, serial_println};
use crate::std::frame::{ColouredChar, Dimensions, Position, special_char, Frame};

/// implement this trait if you require the widget to be able to have an outline
pub trait CgOutline: CgComponent {
	fn render_outline(&self, frame: &mut Frame);
}

/// generic components for the user interface that defined a render method. this should be implemented for all types
/// that can be rendered to the screen.
pub trait CgComponent {
	fn render(&self) -> Result<Frame, RenderError>;
}

/// trait for components that can have editable text, such as search boxes, command palletes, terminals, text inputs etc.
pub trait CgTextEdit: CgComponent {
	fn write_char(&self) -> Result<Frame, RenderError>; // this can also be implemented in a way that inserts characters
	fn delete_char(&self) -> Result<Frame, RenderError>;
	fn move_cursor(&self, direction: bool) -> Result<(), RenderError>; // true = right, false = left
	fn clear(&self) -> Result<Frame, RenderError>;
}











