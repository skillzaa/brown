use std::fs;
use std::path::Path;
use crate::BrownError;
use crate::util;

/// The remove_file method will delete the file on the
/// given path.
/// It is a wrapper around fs::remove_file
/// 
/// # Example
/// ```rust
///  // Make sure the file exists
///  let created = brown::create_file_brute("some_file.md");
/// assert!(created.is_ok());
/// let deleted = brown::remove_file("some_file.md");
/// assert!(deleted.is_ok());   
/// ```

pub fn remove_file(file_path:&str)->Result<bool,BrownError>{
    util::sanitize_file_path(&file_path.to_string())?;
    
let path = Path::new(file_path);
let result  = fs::remove_file(&path);
    match result {
        Ok(()) => return Ok(true),
        Err(_e) => return Err(BrownError::FailedFileDeletion),
    }        
}


mod tests {
    use super::super::*;
    use crate::BrownError;

#[test]
fn basic(){
 let created = create_file_brute("some_file.md");
 assert!(created.is_ok());
 let deleted = remove_file("some_file.md");
 assert!(deleted.is_ok());   
}    
#[test]
fn second(){
 let folders = 
 create_dir_all("rand_dir_77/sub"); 
 assert!(folders.is_ok());
 let created = 
 create_file_brute("rand_dir_77/sub/file.md");
 assert!(created.is_ok());
 let deleted = 
 remove_file("rand_dir_77/sub/file.md");
 assert!(deleted.is_ok()); 
 let cleanup = 
 remove_dir_brute("rand_dir_77");  
 assert!(cleanup.is_ok()); 

}
#[test]
fn url_non_safe_chars_used(){
    let pth = "ab?c.md";
    let r = remove_file(pth);
    let rr = r.unwrap_err();
    assert_eq!(rr,BrownError::NonUrlSafeSymbolFound);
}
#[test]
fn err_begin_with_alphanumeric(){
    let pth = "./abc.md";
    let r = 
    remove_file(pth);
    let rr = r.unwrap_err();
    assert_eq!(rr,BrownError::PathBeginWithAlphabet);
}


}
