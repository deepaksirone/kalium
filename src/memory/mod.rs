// Memory Management submodule 

mod buffer_gap;

#[cfg(ds = "buffer_gap")]
use buffer_gap::*;

use std::result; 

fn InitWorld() -> Result<i32>
{
	unimplemented!();
}

fn SaveWorld(filename: &str) -> Result<i32>
{
	unimplemented!();
}

fn LoadWorld(filename: &str) -> Result<i32>
{
	unimplemented!();
}

fn CreateBuffer(buffer_name: &str) -> Result<Buffer, &'static str> 
{
	unimplemented!();
}

fn DeleteBuffer(buffer_name: &str) -> Result<i32, &'static str>
{
	unimplemented!();
}

fn SetCurrentBuffer(buffer_name: &str) -> Result<Buffer, &'static str>
{
	unimplemented!();
}

fn SetCurrentBufferNext() -> Result<Buffer, &'static str>
{
	unimplemented!();
}
