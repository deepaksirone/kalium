//Editor holds the editor state

use memory::buffer::{BufferList, Buffer, BufferIter};
use std::env;
use std::error::Error;
use std::default::Default;
use std::fs::*;
use std::io::prelude::*;

use rustbox::{Color, RustBox};
use rustbox::*; 

pub struct Editor {
    buf_list: BufferList,
    title: Option<String>,
    cur_buf_idx: usize,
    rustbox: RustBox
}

pub struct Cursor {
    pub x: usize,
    pub y: usize
}

enum FileStatus {
    Ok,
    NotFound,
    Other
}

impl Editor
{
    pub fn init()
    {
        println!("Starting Kalium...");
        let mut editor = Editor::new();
        let mut files: Vec<String> = Vec::new();
        let count = env::args().skip(1).count();

        for argument in env::args().skip(1) {
            match editor.open(argument.as_str()) {
                FileStatus::Ok => { },
                FileStatus::NotFound => { println!("File {} not found", argument); },
                FileStatus::Other =>  { } 
            }
        }
        
//        println!("Width = {}, Height = {}", editor.rustbox.width(), editor.rustbox.height());

        if count == 0 {
            editor.add(Buffer::new_empty_buffer("emp"))
        }
        editor.set_cur_buf(0);
        editor.redraw();  
        
    }


    

    
    fn new() -> Self
    {
        Editor {
            buf_list: BufferList::new(),
            title: None,
            cur_buf_idx: 0,
            rustbox: RustBox::init(Default::default()).ok().unwrap()
        }

    }

    fn open(&mut self, s: &str) -> FileStatus
    {
        if let Some(mut file) = File::open(s).ok() {
            let mut st = String::new();
            let _ = file.read_to_string(&mut st);
            self.buf_list.add(Buffer::new_from_str("bfs", s, st.as_str())); 
        
            FileStatus::Ok
        }
        else {
            FileStatus::NotFound
        }

    }

    fn set_cur_buf(&mut self, idx: usize)
    {
        self.cur_buf_idx = idx; 
    }

    fn add(&mut self, buf: Buffer)
    {
        self.buf_list.add(buf);
    }

    fn current_buffer(&self) -> Option<&Buffer>
    {
        self.buf_list.get_buf(self.cur_buf_idx)
    }

    fn redraw(&mut self)
    {
        for x in 0..self.rustbox.width() {
           self.rustbox.print_char(x, 0, RB_NORMAL, Color::Black, Color::Cyan, '-');
        }
        self.rustbox.set_cursor(0, 1);
        for (index, part) in self.current_buffer().unwrap().to_string().lines().enumerate() {
                self.rustbox.print(0, index + 1, RB_NORMAL, Color::White, Color::Black, part);
        }
        self.rustbox.present();

        loop {
             match self.rustbox.poll_event(false) {
                 Ok(Event::KeyEvent(key)) => {
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



}

impl Cursor
{
    pub fn new() -> Self {
        Cursor {
            x: 0,
            y: 0
        }
    }
}
