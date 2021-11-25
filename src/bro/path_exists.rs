use std::fs;
use std::fs::{ReadDir,DirEntry,File};
use std::io::{Error,ErrorKind};
use std::path::Path;
use crate::bro;
/// The Rust std::fs::DirEntry path object has "exists" however
/// this function takes a &str and converts that into path 
/// thus is useful when the DirEntry object has not been obtained.
pub fn path_exists( value:&str)->bool{
    let path = Path::new(value);
    let tf = path.exists();
    tf
 }
