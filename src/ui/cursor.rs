pub struct Cursor {
    pub x: isize,
    pub y: isize
}

impl Cursor
{
    pub fn new() -> Self {
        Cursor {
            x: 0,
            y: 0
        }
    }

    pub fn get_x(&self) -> isize {
        self.x as isize
    }

    pub fn get_y(&self) -> isize {
        self.y as isize
    }
    
    pub fn set_x(&mut self, x: isize) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: isize) {
        self.y = y;
    }


}

