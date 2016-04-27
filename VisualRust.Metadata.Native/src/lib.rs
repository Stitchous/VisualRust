#![feature(rustc_private)] 

extern crate rustc_metadata;

#[no_mangle]
pub fn get_crate_name(data: &[u8]) -> *const u8
{
    rustc_metadata::decoder::get_crate_name(data).as_ptr()
}