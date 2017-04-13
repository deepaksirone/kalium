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


    fn move_left(&mut self) -> bool {
        unimplemented!();
    }
    
    fn move_right(&mut self) -> bool {
        unimplemented!(); 
    }


}

