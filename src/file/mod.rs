use std::fs::*;
use std::io::prelude::*;
use memory::buffer::{Buffer};
use editor::Editor; 

pub enum FileStatus {
    Ok,
    NotFound,
    Other
}

impl Editor {

    pub fn open(&mut self, path: &str) -> FileStatus
    {
        if let Some(mut file) = File::open(path).ok() {
            let mut con = String::new();
            let _ = file.read_to_string(&mut con);

            let mut new_buffer = Buffer::new_from_str("name", path, con.as_str());
            let new_buf_idx = self.add_buffer(new_buffer);
            self.set_cur_buf(new_buf_idx);
            FileStatus::Ok

        }
        else {
            FileStatus::NotFound
        }

    }
    

}
// fn encrypt
