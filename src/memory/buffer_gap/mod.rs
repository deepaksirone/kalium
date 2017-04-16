use std::collections::{VecDeque};

const BUFSIZE: usize = 4500; 

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

    filename: String,
    modename: String,

    data: Option<Box<GapBuffer>>
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

    pub fn new() -> Self {
        GapBuffer { 
            left: VecDeque::new(),
            right: VecDeque::new(),
            gap_start: 0,
            gap_end: BUFSIZE - 1
        }
    }

    fn pop_left(&mut self) -> Option<char> {
        self.left.pop_back()   
    }

    fn pop_right(&mut self) -> Option<char> {
        self.right.pop_front()
    }
    
    fn push_left(&mut self, c: char) {
        self.left.push_back(c);
    }

    fn push_right(&mut self, c: char) {
        self.right.push_front(c);
    }


    fn move_gap_left(&mut self) -> bool {
        if let Some(c) = self.pop_left() {
            self.push_right(c);
            self.gap_start -= 1;
            self.gap_end -= 1;
            true
        }
        else {
            false
        }

    }
    
    fn move_gap_right(&mut self) -> bool {
        if let Some(c) = self.pop_right() {
            self.push_left(c);
            self.gap_start += 1;
            self.gap_end += 1;
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
        self.gap_start += 1;
    }

    pub fn delete_char(&mut self, point: usize)
    {
        self.move_gap_to_point(point + 1);
        self.pop_left();
        self.gap_start -= 1;
    }
}

#[test]
fn tst()
{
    let mut g = GapBuffer::new();
    g.insert_char('a', 0);
    g.insert_char('b', 0);
    g.insert_char('c', 0);
    g.delete_char(0);

    assert_eq!(g.pop_left(), Some('c'));


    

} 
