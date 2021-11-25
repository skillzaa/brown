use std::fs;
use std::fs::{ReadDir,DirEntry,File};
use std::io::{Error,ErrorKind};
use std::path::Path;
use crate::bro;

/// The create_file_brute function is not safe. It means that it will create a new file even if the old one exists. 
/// If this is not what you want you should try create_file.
pub fn create_file_brute(file_path:&str)->Result<File,Error>{
    File::create(file_path)
    } 
