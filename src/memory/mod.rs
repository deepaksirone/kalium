// Memory Management submodule 

//mod buffer_gap;
mod buffer_gap;
pub mod buffer;

#[cfg(feature = "buffer_gap")]
use memory::buffer::Buffer;

use std::result; 

fn init_world() -> Result<i32, i32>
{
	unimplemented!();
}

fn save_world(filename: &str) -> Result<i32, i32>
{
	unimplemented!();
}

fn load_world(filename: &str) -> Result<i32, i32>
{
	unimplemented!();
}

fn create_buffer(buffer_name: &str) -> Result<Buffer, &'static str> 
{
	unimplemented!();
}

fn delete_buffer(buffer_name: &str) -> Result<i32, &'static str>
{
	unimplemented!();
}

fn set_current_buffer(buffer_name: &str) -> Result<Buffer, &'static str>
{
	unimplemented!();
}

fn set_current_buffer_next() -> Result<Buffer, &'static str>
{
	unimplemented!();
}
