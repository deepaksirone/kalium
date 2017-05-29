//Editor holds the editor state

use memory::buffer::{BufferList, Buffer, BufferIter};
use std::env;
use std::fs::File;
use std::io::prelude::*;

pub struct Editor {
    buf_list: BufferList,
    title: Option<String>,
    cur_buf_idx: usize

}

impl Editor
{
    pub fn init()
    {
        println!("Starting Kalium...");
        for argument in env::args() {
            println!("Attempting to open: {}", argument);
            match File::open(argument) {
                Ok(val) => { },
                Err(e) => println!("{}", e)
            }
        }
    }
    
    

}

