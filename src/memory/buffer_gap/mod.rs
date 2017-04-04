pub struct BufferList
{
    head: Option<Box<Buffer>>,
}

struct Buffer
{
    name: String,
    nextentry: Option<Box<Buffer>>,

    point: u64,
    modified: u8,
    length: u64,

    filename: String,
    modename: String,

    data: Option<Box<GapBuffer>>
}


struct GapBuffer
{





