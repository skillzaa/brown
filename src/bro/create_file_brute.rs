use std::fs::{File};
use crate::BrownError;
/// The create_file_brute function is not safe. It means that it will create a new file even if an old one exists. 
/// If this is not what you want you should try create_file.
pub fn create_file_brute(file_path:&str)->Result<File,BrownError>{
    let r = File::create(file_path);
    match r {
    Ok(item)=>{
    return Ok(item);
    },
    Err(_e)=>{return Err(BrownError::FailedFileCreation);},
    }
    } 
