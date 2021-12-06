use std::fs;
use std::io::{Error};
use std::path::Path;
/// The **remove_dir_brute** fn will delete a folder even if it has other files and folders. USE WITH CAUTION!!
/// Example :: 
/// ```rust
/// let p_dir = brown::create_dir("parent");
/// let outcome = brown::remove_dir_brute("parent");
///  assert!(outcome.is_ok());
/// ``` 
pub fn remove_dir_brute( dir_name:&str)->Result<bool,Error> {
    let complete = String::from("./") + &dir_name;
    let path = Path::new(&complete);
        match fs::remove_dir_all(path) {
            Ok(_ok)=>{
                Ok(true)
            },
            Err(e)=>{
                Err(e)
            },
        }
}