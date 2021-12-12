use std::fs;
use std::path::Path;
use crate::BrownError;
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
pub fn remove_dir( dir_name:&str)->Result<bool,BrownError> {
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