use std::fs;
use std::io::{Error};

/// This function will create a file if it does not exist, and 
/// will entirely replace its contents if it does.
pub fn write_file(path:&str,content:&String)->Result<bool,Error>{
    fs::write(path, content)?;
    Ok(true)
  }
  
  