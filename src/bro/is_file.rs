use std::fs::{DirEntry};
use std::io::{Error};
/// The is_file function takes in Rust Struct &DirEntry and return if that &DirEntry is a file or a folder. It is for ease of use and to save us duplicate code. 
pub fn is_file(entry:&DirEntry)->Result<bool,Error>{
    let file_type = entry.file_type();
    match file_type {
        Ok(ft)=>{
            return Ok(ft.is_file());
        },
        Err(e)=> return Err(e),
    }
}