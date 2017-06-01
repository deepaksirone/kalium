
#[cfg(feature = "buffer_gap")] 
use memory::buffer_gap::{GapBuffer, GapBufferIter};

use std::path::PathBuf;
use std::io::prelude::*;
use std::fs::{File, OpenOptions};
use std::io::*;



pub struct BufferList
{
    head: Vec<Buffer>
}

pub struct Buffer
{
    name: String,

    point: usize,
    modified: usize,
//  length: usize,

    filename: Option<PathBuf>,
    modename: Option<String>,

    #[cfg(feature = "buffer_gap")]
    data: Option<Box<GapBuffer>>
}

pub struct BufferIter<'a>
{
    #[cfg(feature = "buffer_gap")]
    iter: GapBufferIter<'a>
}

impl<'a> Iterator for BufferIter<'a>
{
    type Item = char;
    fn next(&mut self) -> Option<Self::Item>
    {
        self.iter.next()
    }

}


impl Buffer 
{
    pub fn new_from_str(name: &str,fname: &str, s: &str) -> Buffer
    {
        Buffer { 
            name: name.to_owned(),
            point: 0,
            modified: 0,
//            length: filesize,
            filename: Some(PathBuf::from(fname)),
            modename: None,

            #[cfg(feature = "buffer_gap")]
            data: Some(Box::new(GapBuffer::new_from_str(s)))
    
        }
            
    }
    
    pub fn new_empty_buffer(name: &str) -> Buffer
    {
        Buffer {
            name: name.to_owned(),
            point: 0,
            modified: 0,
            filename: None,
            modename: None,

            #[cfg(feature = "buffer_gap")]
            data: Some(Box::new(GapBuffer::new(1)))
        }
    }


    pub fn set_filename(&mut self, fname: &str) 
    {
        self.filename = Some(PathBuf::from(fname));   
    }

    pub fn get_filename(&self) -> Option<PathBuf>
    {
        self.filename.as_ref().map(|fname| fname.clone())
    }

    pub fn write_buffer(&self) -> bool 
    {
        let ops = OpenOptions::new().create(true).write(true).open(self.filename.as_ref().map(|fname| fname.clone()).unwrap());
        let mut f = BufWriter::new(ops.ok().unwrap());
        let mut b = [0; 4];
        for chr in self.data.as_ref().map(|buf| buf.iter()).unwrap() {
            f.write(chr.encode_utf8(&mut b).as_bytes()).unwrap();
        }
        true
    }

    pub fn read_buffer(&mut self) -> bool
    {
        let ops = OpenOptions::new().read(true).open(self.filename.as_ref().map(|fname| fname.clone()).unwrap());
        let mut f = BufReader::new(ops.ok().unwrap());
        let mut s = String::new();
        f.read_to_string(&mut s);

        self.data.as_mut().map(|buf| buf.read_from_str(&s));
        true

    }

    pub fn set_modified(&mut self) 
    {
        self.modified = 1;
    }

    pub fn get_modified(&self) -> usize 
    {
        self.modified
    }

    pub fn set_point_abs(&mut self, point: usize) -> bool 
    {
        if point >= self.get_length() {
            false
        }
        else {
            self.point = point; 
            true
        }
    }

    pub fn set_point_rel(&mut self, offset: i64) -> bool
    {
        let p = self.point as i64; 
        if p + offset < 0 || p + offset >= self.get_length() as i64 { 
            false 
        }
        else {
            self.point = (p + offset) as usize;
            true
        }
    }

    pub fn get_point(&mut self) -> usize
    {
        self.point
    }

    pub fn get_length(&self) -> usize 
    {
        self.data.as_ref().map(|boxed_buf_ref| boxed_buf_ref.get_length()).unwrap()
    }

    pub fn insert_string(&mut self, s: &str, pos: usize)
    {
/*       self.data = self.data.take().map(|boxed_buf| {
                let mut buf = *boxed_buf; 
                buf.insert_string(s, pos);
                Box::new(buf)
        });
*/
        self.data.as_mut().map(|boxed_buf_ref| {
                 boxed_buf_ref.insert_string(s, pos);
        }); 
    }

    pub fn delete_chars(&mut self, pos: usize, count: usize)
    {
/*        self.data = self.data.take().map(|boxed_buf| {
                let mut buf = *boxed_buf;
                buf.delete_chars(pos, count);
                Box::new(buf)
        });
*/
        self.data.as_mut().map(|boxed_buf_ref| {
                 boxed_buf_ref.delete_chars(pos, count);
        });
        
    }

/*
    pub fn get_string_abs(&self, pos: usize, length: usize) -> String
    {
        unimplemented!();
    }

    pub fn get_string_rel(&self, length: usize) -> String
    {
        unimplemented!();
    }
*/
/*    fn increase_buffer_size(&mut self, num_chars: usize)
    {
        self.data.as_mut().map(|boxed_buf_ref| {
                boxed_buf_ref.increase_buffer_size(num_chars);
        });
    }
*/
    pub fn to_string(&self) -> String
    {
        let mut s = String::new();
        for chr in self.data.as_ref().map(|buf| buf.iter()).unwrap() {
            s.push(chr);
        }
        s
    }
        

}

pub trait BufferTrait {

    fn new(size: usize) -> Self;

    fn new_from_str(s: &str) -> Self;

    fn insert_string(&mut self, s: &str, pos: usize); 

    fn delete_chars(&mut self, pos: usize, count: usize);

    fn get_length(&self) -> usize; 

}

impl BufferList {

    pub fn new() -> Self {
        BufferList {
            head: Vec::new()
        }
    }
    
    pub fn add(&mut self, buf: Buffer) {
        self.head.push(buf);
    }


}

