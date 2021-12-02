use std::fs;
use std::io::{Error};
use std::path::Path;

/// The remove_file method will delete the file on the
/// given path.
/// It is a wrapper around fs::remove_file
pub fn remove_file(file_path:&str)->Result<bool,Error>{
    let path = Path::new(file_path);
        let result  = fs::remove_file(&path);
        match result {
            Ok(()) => return Ok(true),
            Err(e) => return Err(e),
        }        
}


