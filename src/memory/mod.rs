// Memory Management submodule 

pub mod buffer_gap;

#[cfg(ds = "buffer_gap")]
use buffer_gap::*;


fn InitWorld() -> i32
{
	unimplemented!();
}

fn SaveWorld(filename: &str) -> i32
{
	unimplemented!();
}

fn LoadWorld(filename: &str) -> i32
{
	unimplemented!();
}

fn CreateBuffer(buffer_name: &str) -> i32 
{
	unimplemented!();
}

fn DeleteBuffer(buffer_name: &str) -> i32
{
	unimplemented!();
}

fn SetCurrentBuffer(buffer_name: &str) -> i32
{
	unimplemented!();
}

fn SetCurrentBufferNext() -> i32
{
	unimplemented!();
}
