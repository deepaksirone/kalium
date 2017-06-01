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
            editor.open(argument.as_str());
        }
        
        println!("Count = {}", count);

        if count == 0 {
            editor.add(Buffer::new_empty_buffer("emp"))
        }
        editor.set_cur_buf(0);
    
        
    }


    
    
    fn new() -> Self
    {
        Editor {
            buf_list: BufferList::new(),
            title: None,
            cur_buf_idx: 0
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



}

