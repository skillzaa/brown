use std::fs;
use std::io::{Error};
use std::path::Path;

/// The remove_dir funtion will remove a directory only of its empty.
/// Its operation is safe. This fn should be used normally unless brute removal is required 
/// Example :: 
/// ```rust
/// // Ensure there is no old folder
/// let _ = brown::remove_dir_brute("parent");
/// let p_dir = brown::create_dir("parent");
/// let outcome = brown::remove_dir("parent");
///  assert!(outcome.is_ok());
/// ``` 
pub fn remove_dir( dir_name:&str)->Result<bool,Error> {
    let complete = String::from("./") + &dir_name;
    let path = Path::new(&complete);
        match fs::remove_dir(path) {
            Ok(_ok)=>{
                Ok(true)
            },
            Err(e)=>{
                Err(e)
            },
        }
}