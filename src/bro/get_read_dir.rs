use std::fs;
use std::fs::{ReadDir,DirEntry,File};
use std::io::{Error,ErrorKind};
use std::path::Path;
use crate::bro;

/// The get_read_dir will return "ReadDir" struct from Rust which
/// is a iterator over the directory  
pub fn get_read_dir(dir_path:&str)->Result<ReadDir,Error>{
    let complete = String::from("./") + &dir_path;
    let path_com = Path::new(&complete);
    let read_dir = fs::read_dir(path_com);
    match read_dir {
        Ok(read_dir_ok)=>{
            return Ok(read_dir_ok);
        },
        Err(_e)=>{
            let e = Error::new(ErrorKind::NotFound, "failed to read the directory, it may not exist or the path may be wrong");
            return Err(e);
        },
    }
}    