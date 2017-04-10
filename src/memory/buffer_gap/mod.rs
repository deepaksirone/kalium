const BUFSIZE: usize = 4500; 

pub struct BufferList
{
    head: Vec<Box<BufferDescriptor>>,
}

struct BufferDescriptor
{
    name: String,
    nextentry: Option<Box<Buffer>>,

    point: usize,
    modified: usize,
    length: usize,

    filename: String,
    modename: String,

    data: Option<Box<GapBuffer>>
}


struct GapBuffer
{
   left: Vec<char>,
   right: Vec<char>,
    
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
            left: Vec::new(),
            right: Vec::new(),
            gap_start: 0,
            gap_end: BUFSIZE - 1
        }
    }

    fn pop_left() -> Option<char> {
        unimplemented!();   
    }

    fn pop_right() -> Option<char> {
        unimplemented!(); 
    }

    fn move_left(&mut self) -> bool {

    }
    
    fn move_right(&mut self) -> bool {

    }


}

