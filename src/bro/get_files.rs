use std::fs;
use std::fs::{ReadDir,DirEntry,File};
use std::io::{Error,ErrorKind};
use std::path::Path;
use crate::bro;
/// The get_files will get all the files from a folder leaving out the directories.
/// It will return error if no file is found thus the user does not have to check if the returned vec has some values or not. 
pub fn get_files(dir_path:&str)->Result<Vec<DirEntry>,Error>{
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
        let e = Error::new(ErrorKind::NotFound,"found no files in the directory");
        return Err(e);
    }else {
        return Ok(vec);
    }
}