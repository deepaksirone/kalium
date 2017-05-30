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
        for (i, argument) in env::args().enumerate() {
            if i > 0 { 
                println!("Attempting to open: {}", argument);
                match File::open(argument) {
                    Ok(val) => {},
                    Err(e) => println!("{}", e)
                }
            } 
        }
    }
    
    fn new() -> Self
    {
        Editor {
            buf_list: BufferList::new(),
            title: None,
            cur_buf_idx: 0
        }

    }

}

