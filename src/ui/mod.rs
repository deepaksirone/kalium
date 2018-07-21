pub mod cursor;
use rustbox::{Color, RustBox};
use rustbox::*;


pub struct Ui {
    // Struct for UI
    cur_buf_idx: usize,
    // RustBox for rendering
    rustbox: RustBox,
    // This is the way to go!

}


impl Ui 
{

    pub fn render(&mut self)
    {
        unimplemented!();
    }

    pub fn ui_loop(&mut self)
    {
        unimplemented!();
    }

}

