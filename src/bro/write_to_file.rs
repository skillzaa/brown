use std::fs;
use crate::BrownError;
use std::path::Path;
use crate::util;
/// This function will create a file if it does not exist, and 
/// will entirely replace its contents if it does.
pub fn write_to_file(path:&str,content:&String)->Result<bool,BrownError>{
    
    util::sanitize_file_path(&path.to_string())?;

    let complete = String::from("./") + &path;
    let path2 = Path::new(&complete);
//.........
    let result  = fs::write(path2, content);
        match result {
            Ok(()) => return Ok(true),
            Err(_e) => return Err(BrownError::FailedFileCreation),
        }
}

mod tests {
use super::super::*;
use crate::BrownError;
use super::*;

#[test]
fn normal_write_to_file(){
    let pth = "abc.md";
    let blob = "1234567890";
    let r = 
    write_to_file(pth,&blob.to_string());
    assert!(r.is_ok());
    let j = remove_file(pth);
    assert!(j.is_ok());

}
#[test]
fn url_non_safe_chars_used(){
    let pth = "ab??c.md";
    let blob = "1234567890";
    let r = 
    write_to_file(pth,&blob.to_string());
    let rr = r.unwrap_err();
    assert_eq!(rr,BrownError::NonUrlSafeSymbolFound);
}
#[test]
fn err_begin_with_alphanumeric(){
    let pth = "./abc.md";
    let blob = "1234567890";
    let r = 
    write_to_file(pth,&blob.to_string());
    let rr = r.unwrap_err();
    assert_eq!(rr,BrownError::PathBeginWithAlphabet);
}

}
  
  