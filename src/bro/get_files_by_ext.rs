use std::fs::{DirEntry};
use std::io::{Error,ErrorKind};
use crate::bro;

/// The get_files_by_ext is just like get_files but it get files based on their extention.
/// Do not add "." (the dot) with file extention
/// Example::
/// "md" , "html" , "txt" etc
/// Do not include the "./" before the dir_path
pub fn get_files_by_ext(dir_path:&str,ext:&str)->Result<Vec<DirEntry>,Error>{
    let mut vec:Vec<DirEntry> = Vec::new() ;
    let files  = bro::get_files(dir_path)?;
    for file in files {
        let file_ext = bro::get_ext(&file)?;
            if file_ext == ext {
                    vec.push(file);
            }else {
                continue;
            }
        
    }
    if vec.len() <= 0 {
        let e = Error::new(ErrorKind::NotFound,"found no files with the given extention");
        return Err(e);
    }else {
        return Ok(vec);
    }
}