use std::fs;
use std::path::Path;
use crate::BrownError;
use crate::util;

/// The remove_dir funtion will remove a directory only if its empty.
/// Its operation is safe. This fn should be used always unless brute removal is required 
/// Example :: 
/// ```rust
/// // Ensure there is no old folder
/// let _ = brown::remove_dir_brute("parent");
/// let p_dir = brown::create_dir("parent");
/// let outcome = brown::remove_dir("parent");
///  assert!(outcome.is_ok());
/// ``` 
pub fn remove_dir(dir_name:&str)->Result<bool,BrownError> {
util::sanitize_dir_path(&dir_name.to_string())?;
  
    let complete = String::from("./") + &dir_name;
    let path = Path::new(&complete);
        match fs::remove_dir(path) {
            Ok(_ok)=>{
                Ok(true)
            },
            Err(_e)=>{
                Err(BrownError::FailedDirDeletion)
            },
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
 create_dir_all("somefolder/sub"); 
 assert!(folders.is_ok());
 let created = 
 create_file_brute("somefolder/sub/file.md");
 assert!(created.is_ok());
 let deleted = 
 remove_file("somefolder/sub/file.md");
 assert!(deleted.is_ok()); 
 let cleanup = 
 remove_dir_brute("somefolder");  
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
