use std::fs;
use std::fs::{ReadDir,DirEntry,File};
use std::io::{Error,ErrorKind};
use std::path::Path;
use crate::bro;


pub fn is_file(entry:&DirEntry)->Result<bool,Error>{
    let file_type = entry.file_type();
    match file_type {
        Ok(ft)=>{
            return Ok(ft.is_file());
        },
        Err(e)=> return Err(e),
    }
}