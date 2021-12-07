use std::fs::{DirEntry};
use crate::bro;
use crate::BrownError;
/// The get_files will get all the files from a folder leaving out the directories.
/// It will return error if no file is found thus the user does not have to check if the returned vec has some values or not. 
pub fn get_files(dir_path:&str)->Result<Vec<DirEntry>,BrownError>{
    let mut vec:Vec<DirEntry> = Vec::new() ;
    let entries  = bro::get_entries(dir_path)?;
    for entry in entries {
        let is_file = bro::is_file(&entry);
        match is_file {
            Ok(true) => {
                vec.push(entry)
            },
            _=> continue,
        }
    }
    if vec.len() <= 0 {
        return Err(BrownError::DirEmpty);
    }else {
        return Ok(vec);
    }
}