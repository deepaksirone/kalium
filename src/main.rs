#![feature(struct_field_attributes)]
extern crate rustbox;

use std::error::Error;
use std::default::Default;
use std::fs::*;
use std::io::prelude::*; 

use rustbox::{Color, RustBox};
use rustbox::Key;


pub mod memory;

fn main()
{
    let rustbox = RustBox::init(Default::default()).ok().unwrap();
    
    let mut file = std::io::BufReader::new(File::open("test.txt").ok().unwrap());
    let mut s: String = String::new();
//    let _ = file.read_to_string(&mut s);
    for (index, line) in file.lines().enumerate() {
        rustbox.print(1, index + 1, rustbox::RB_BOLD, Color::White, Color::Black, line.ok().unwrap().as_str());     
    }
    rustbox.present();

    loop {
       match rustbox.poll_event(false) {
             Ok(rustbox::Event::KeyEvent(key)) => {
                     match key {
                          Key::Char('q') => { break; }
                          _ => { }
                    }
             },
             Err(e) => panic!("{}", e.description()),
             _ => { }
         }
    }

}
