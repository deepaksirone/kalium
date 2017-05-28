//Editor holds the editor state

use memory::buffer::{BufferList, Buffer, BufferIter};

struct Editor {
    buf_list: BufferList,
    title: Option<String>,
    cur_buf_idx: usize

}

impl Editor
{
    pub fn init()
    {
        unimplemented!();
    }


}

