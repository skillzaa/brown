use std::fs;
use std::fs::{ReadDir,DirEntry,File};
use std::io::{Error,ErrorKind};
use std::path::Path;
use crate::bro;

/// The remove_dir funtion will remove a directory only of its empty.
/// Its operation is safe. This fn should be used normally unless brute removal is required 
/// Example :: ```
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