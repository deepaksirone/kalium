//Editor holds the editor state

use memory::buffer::{BufferList, Buffer, BufferIter};
use std::env;
use std::error::Error;
use std::default::Default;
use std::char::from_digit;

use file::FileStatus; 
use rustbox::{Color, RustBox};
use rustbox::*; 

pub struct Editor {
    buf_list: BufferList,
    title: Option<String>,
    cur_buf_idx: usize,
    rustbox: RustBox
}

/* ui 
pub struct Cursor {
    pub x: isize,
    pub y: isize
}

File

*/

impl Editor
{
    pub fn init()
    {
        println!("Starting Kalium...");

        println!("Char = {}", from_digit((2 as usize) as u32, 10).unwrap());
        let mut editor = Editor::new();
        let mut files: Vec<String> = Vec::new();
        let count = env::args().skip(1).count();

        for argument in env::args().skip(1) {
            match editor.open(argument.as_str()) {
                FileStatus::Ok => { },
                FileStatus::NotFound => { println!("File {} not found", argument); },
                FileStatus::Other =>  { } 
            }
        }
        
//        println!("Width = {}, Height = {}", editor.rustbox.width(), editor.rustbox.height());

        if count == 0 {
            editor.add_buffer(Buffer::new_empty_buffer("emp"));
        }
/* TODO: Implement/redesign this        
        editor.set_cur_buf(0);
        editor.set_cursor(1, 1);
        editor.update_cursor(1, 1);
        editor.redraw();
        
        loop {
             let x = editor.get_cursor().unwrap().get_x();
             let y = editor.get_cursor().unwrap().get_y();
             match editor.rustbox.poll_event(false) {
                 Ok(Event::KeyEvent(key)) => {
                         match key {
                              Key::Right => { editor.set_cursor(x + 1, y); editor.rustbox.present();editor.update_cursor(x + 1, y); },
                              Key::Char('q') => { break; },
                              _ => { }
                         }
                },
                Err(e) => panic!("{}", e.description()),
                        _ => { }
            }
        }
*/ 
    }


    

    
    fn new() -> Self
    {
        Editor {
            buf_list: BufferList::new(),
            title: None,
            cur_buf_idx: 0,
            rustbox: RustBox::init(Default::default()).ok().unwrap()
        }

    }



    pub fn set_cur_buf(&mut self, idx: usize)
    {
        self.cur_buf_idx = idx; 
    }

    pub fn add_buffer(&mut self, buf: Buffer) -> usize
    {
        self.buf_list.add(buf);
        self.buf_list.head.len() - 1
    }

    fn current_buffer(&self) -> Option<&Buffer>
    {
        self.buf_list.get_buf(self.cur_buf_idx)
    }
    
    fn current_buffer_mut(&mut self) -> Option<&mut Buffer>
    {
        self.buf_list.get_buf_mut(self.cur_buf_idx)
    }
/* UI 
    fn redraw_status_bar(&mut self)
    {
        let mut z: String;
        for x in 0..self.rustbox.width() - 1 {
           self.rustbox.print_char(x, 0, RB_NORMAL, Color::Black, Color::Cyan, '-');
        }

        for y in 0..self.rustbox.height() - 1 {
           z = y.to_string();
           self.rustbox.print(0, y + 1, RB_NORMAL, Color::Yellow, Color::Black, z.as_str());
        }
        self.rustbox.present();
    }

    fn redraw(&mut self)
    {
//      self.redraw_status_bar();

        for (index, part) in self.current_buffer().unwrap().to_string().lines().skip(self.scroll_x()).enumerate() {
                self.rustbox.print(1, index + 1, RB_NORMAL, Color::White, Color::Black, part);
        }
        self.rustbox.present();

  
    }

    fn scroll_x(&self) -> usize
    {
        self.current_buffer().unwrap().get_scroll_x()
    }

    fn scroll_y(&self) -> usize
    {
        self.current_buffer().unwrap().get_scroll_y()
    }
    
    fn get_width(&self) -> usize
    {
        self.rustbox.width()
    }

    fn get_height(&self) -> usize
    {
        self.rustbox.height()
    }

    #[inline]
    fn get_cursor(&self) -> Option<&Cursor>
    {
        self.current_buffer().unwrap().get_cursor()
    }

    fn set_cursor(&mut self, x: isize, y: isize)
    {
        self.rustbox.set_cursor(x as isize, y as isize);
    }

    fn update_cursor(&mut self, x: isize, y: isize)
    {
        self.current_buffer_mut().map(|buf| buf.update_cursor(x, y));
    }

    #[inline]
    fn handle_key_event(&mut self, key: Key)
    {
        let x;
        let y;
        {
            let cursor = self.get_cursor().unwrap();
            x = cursor.get_x();
            y = cursor.get_y(); 
        }
        match key {
            Key::Tab => {
                    self.set_cursor(x + 4, y); self.rustbox.present(); 
                    self.update_cursor(x + 4, y);
            },

            _ => { } 
        }

    }

    fn handle_resize_event(&mut self, x: i32, y: i32)
    {
        unimplemented!();
    }

    fn handle_mouse_event(&mut self, mouse: Mouse, x: i32, y: i32)
    {
        unimplemented!();
    }

    fn event_loop(&mut self)
    { 
        loop {
             match self.rustbox.poll_event(false)
             {
                    Ok(Event::KeyEvent(key)) => self.handle_key_event(key),
                    Ok(Event::ResizeEvent(x, y)) => self.handle_resize_event(x, y),
                    Ok(Event::MouseEvent(mouse, x, y)) => self.handle_mouse_event(mouse, x, y),
                    _ => { }
             }
        }

    }
*/
}

/* UI 
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
*/
