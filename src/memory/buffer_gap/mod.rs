use std::collections::vec_deque::*;


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
