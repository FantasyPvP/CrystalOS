use crate::{
    kernel::render::{Color, write, RENDERER, BUFFER_WIDTH, BUFFER_HEIGHT},
    kernel::tasks::keyboard::KEYBOARD,
    kernel::os::OS,
    shell::{Application, Error, CMD }
};
use alloc::{boxed::Box, string::{String, ToString}, vec::Vec};


pub use crate::{println, print, serial_print, serial_println};

pub async fn stdin() -> String {
    let string = KEYBOARD.lock().get_string().await;
    string
}

pub async fn stdchar() -> char {
    let chr = KEYBOARD.lock().get_keystroke().await;
    chr
}


#[derive(Clone, Copy)]
pub struct Frame {
    pub frame: [ [ char; BUFFER_WIDTH ]; BUFFER_HEIGHT]
}

pub fn render_frame(frame: Frame) {
    RENDERER.lock().render_frame(frame.frame)
}