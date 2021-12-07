use std::fs::{DirEntry};
use std::io::{Error};

/// We can directly get is_dir function using DirEntry. However this saves us digging down two levels.
pub fn is_dir(entry:&DirEntry)->Result<bool,Error>{
    let file_type = entry.file_type();
    match file_type {
        Ok(ft)=>{
            return Ok(ft.is_dir());
        },
        Err(e)=> return Err(e),
    }
}
