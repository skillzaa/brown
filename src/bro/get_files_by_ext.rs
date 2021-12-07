use std::fs::{DirEntry};
use crate::bro;
use crate::BrownError;
/// The get_files_by_ext is just like get_files but it get files based on their extention. If no file is found with the given extention an error is returned.
/// 
/// Do not add "." (the dot) with file extention
/// e.g "md" , "html" , "txt" etc
/// 
/// Do not include the "./" before the dir_path

pub fn get_files_by_ext(dir_path:&str,ext:&str)->Result<Vec<DirEntry>,BrownError>{
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
        return Err(BrownError::FileExtError);
    }else {
        return Ok(vec);
    }
}