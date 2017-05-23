
#[cfg(feature = "buffer_gap")] 
use memory::buffer_gap::{GapBuffer, GapBufferIter};

pub struct BufferList
{
    head: Vec<Box<Buffer>>,
}

pub struct Buffer
{
    name: String,

    point: usize,
    modified: usize,
//  length: usize,

    filename: Option<String>,
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
            filename: Some(fname.to_owned()),
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
            data: Some(Box::new(GapBuffer::new(1)))
        }
    }


    fn set_filename(&mut self, fname: &str) 
    {
        self.filename = Some(fname.to_owned());   
    }

    fn get_filename(&self) -> Option<String>
    {
        self.filename.as_ref().map(|fname| fname.clone())
    }

    fn write_buffer(&self) -> bool 
    {
        unimplemented!();
    }

    fn read_buffer(&mut self) -> bool
    {
        unimplemented!()        
    }

    fn set_modified(&mut self) 
    {
        self.modified = 1;
    }

    fn get_modified(&self) -> usize 
    {
        self.modified
    }

    fn set_point_abs(&mut self, point: usize) -> bool 
    {
        if point >= self.data.as_mut().map(|boxed_buf_ref| boxed_buf_ref.length).unwrap() {
            false
        }
        else {
            self.point = point; 
            true
        }
    }

    fn set_point_rel(&mut self, offset: i64) -> bool
    {
        let p = self.point as i64; 
        if p + offset < 0 || p + offset >= self.data.as_mut().map(|boxed_buf_ref| boxed_buf_ref.length).unwrap() as i64 { 
            false 
        }
        else {
            self.point = (p + offset) as usize;
            true
        }
    }

    fn get_point(&mut self) -> usize
    {
        self.point
    }

    fn get_length(&self) -> usize 
    {
        self.data.as_ref().map(|boxed_buf_ref| boxed_buf_ref.length).unwrap()
    }

    fn insert_string(&mut self, s: &str, pos: usize)
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

    fn delete_chars(&mut self, pos: usize, count: usize)
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

    fn get_string_abs(&self, pos: usize, length: usize) -> String
    {
        unimplemented!();
    }

    fn get_string_rel(&self, length: usize) -> String
    {
        unimplemented!();
    }

/*    fn increase_buffer_size(&mut self, num_chars: usize)
    {
        self.data.as_mut().map(|boxed_buf_ref| {
                boxed_buf_ref.increase_buffer_size(num_chars);
        });
    }
*/

}


