use std::collections::vec_deque::*;
use std::fs::metadata;

pub struct BufferList
{
    head: Vec<Box<Buffer>>,
}

pub struct Buffer
{
    name: String,

    point: usize,
    modified: usize,
    length: usize,

    filename: Option<String>,
    modename: Option<String>,

    data: Option<Box<GapBuffer>>
}

struct GapBufferIter<'a>
{
    left_iter: Iter<'a, char>,
    right_iter: Iter<'a, char> 
}

struct GapBuffer
{
   left: VecDeque<char>,
   right: VecDeque<char>,
    
   gap_start: usize,
   gap_end:   usize,

}

impl GapBuffer
{
    fn user_to_gap(&self, user_point: usize) -> usize {
        if user_point <= self.gap_start {
            user_point
        }
        else {
            (self.gap_end - self.gap_start) + user_point
        }
    }

    pub fn new(size: usize) -> Self {
        GapBuffer { 
            left: VecDeque::new(),
            right: VecDeque::new(),
            gap_start: 0,
            gap_end: size - 1
        }
    }

    fn pop_left(&mut self) -> Option<char> {
        self.left.pop_back().map(|chr| { 
            self.gap_start -= 1;
            chr 
        })
    }

    fn pop_right(&mut self) -> Option<char> {
        self.right.pop_front().map(|chr| {
            self.gap_end += 1;
            chr
        })
    }
    
    fn push_left(&mut self, c: char) {
        self.left.push_back(c);
        self.gap_start += 1;
    }

    fn push_right(&mut self, c: char) {
        self.right.push_front(c);
        self.gap_end -= 1;
    }


    fn move_gap_left(&mut self) -> bool {
        if let Some(c) = self.pop_left() {
            self.push_right(c);
            true
        }
        else {
            false
        }

    }
    
    fn move_gap_right(&mut self) -> bool {
        if let Some(c) = self.pop_right() {
            self.push_left(c);
            true
        }
        else {
            false
        }
    }
    
    fn move_gap_to_point(&mut self, point: usize)
    {
        let pos = self.user_to_gap(point);
        if pos <= self.gap_start {
            for i in 1..(self.gap_start - pos) + 1 {
                self.move_gap_left();
            }
        }
        else {
            for i in 1..(pos - self.gap_end) + 1 {
                self.move_gap_right();
            }
        }

        

    }
    pub fn insert_char(&mut self, c: char, point: usize)
    {
        self.move_gap_to_point(point);
        self.push_left(c);
    }

    pub fn delete_char(&mut self, point: usize)
    {
        self.move_gap_to_point(point + 1);
        self.pop_left();
    }

    pub fn insert_string(&mut self, s: &str, point: usize)
    {
        let mut p = point; 
        for chr in s.chars() {
            self.insert_char(chr, p);
            p += 1;
        }
    }
    
    pub fn delete_chars(&mut self, point: usize, n_chars: usize)
    {
        let mut p = point;
        for i in 1..n_chars + 1 {
            self.delete_char(p);
            p -= 1;
        }
    }

    pub fn iter(&self) -> GapBufferIter
    {
        GapBufferIter { 
            left_iter: self.left.iter(),
            right_iter: self.right.iter()
        }
    }


}

impl<'a> Iterator for GapBufferIter<'a>
{
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        match self.left_iter.next() {
            Some(c) => Some(*c),
            None => self.right_iter.next().map(|chr_ref| *chr_ref)
        }
    }

}

impl Buffer 
{
    pub fn new(name: &str, fname: &str) -> Buffer
    {
        let filesize = metadata(name).ok().unwrap().len() as usize; 
        Buffer { 
            name: name.to_owned(),
            point: 0,
            modified: 0,
            length: filesize,
            filename: Some(fname.to_owned()),
            modename: None,
            data: Some(Box::new(GapBuffer::new(filesize)))
    
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
        unimplemented!();
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
        if point >= self.length {
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
        if p + offset < 0 || p + offset >= self.length as i64 { 
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
        self.length
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




}



#[test]
fn tst()
{
    let mut g = GapBuffer::new(1000);
    let s: &str = "tab";
    let t = "cab";

//    g.insert_char('a', 0);
//    g.insert_char('b', 0);
//    g.insert_char('c', 0);
//    g.delete_char(0);
//    g.insert_char('d', 1);
//    g.delete_char(1);
    g.insert_string(&s, 0);
    g.insert_string(&t, 0);

    println!("{:?} {:?}", g.left, g.right);
    let mut itr = g.iter();
    assert_eq!(itr.next(), Some('c'));
    assert_eq!(itr.next(), Some('a'));
    assert_eq!(itr.next(), Some('b'));
    assert_eq!(itr.next(), Some('t'));
    assert_eq!(itr.next(), Some('a'));
    assert_eq!(itr.next(), Some('b'));
    assert_eq!(itr.next(), None);
    
//    assert_eq!(g.pop_left(), Some('t'));


    

} 
