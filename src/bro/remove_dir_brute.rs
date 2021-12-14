use std::fs;
use std::path::Path;
use crate::BrownError;
use crate::util;
/// The **remove_dir_brute** fn will delete a folder even if it has other files and folders. USE WITH CAUTION!!
/// Example :: 
/// ```rust
/// let _ = brown::remove_dir_brute("base_folder");
/// let p_dir = brown::create_dir("base_folder");
/// let outcome = brown::remove_dir_brute("base_folder");
///  assert!(outcome.is_ok());
/// ``` 
pub fn remove_dir_brute(dir_name:&str)->Result<bool,BrownError> {
    util::sanitize_dir_path(&dir_name.to_string())?;  
    let complete = String::from("./") + &dir_name;
    let path = Path::new(&complete);
        match fs::remove_dir_all(path) {
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
 let created = create_dir_brute("dir_99");
 assert!(created.is_ok());
 let deleted = remove_dir_brute("dir_99");
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
