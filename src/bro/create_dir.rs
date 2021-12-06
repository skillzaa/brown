use std::fs;
use std::io::{Error,ErrorKind};
use std::path::Path;

use crate::bro_path::BroPath;
/// The create_dir function will create a folder safely i.e in case the folder does not exist, it will be create But incase the folder already exists the function will return an error.
/// # Example (folder does not exist)
/// ```rust
/// let p_dir = brown::create_dir("random_folder");
///  assert!(p_dir.is_ok());
/// // clean up
/// let clean = brown::remove_dir_brute("random_folder");
/// assert!(clean.is_ok());
/// ``` 
/// # Example (folder already exists)
/// ```rust
/// let p_dir = brown::create_dir("random_folder");
///  assert!(p_dir.is_ok());
/// let p_dir_again = brown::create_dir("parent");
///  assert!(p_dir_again.is_err());
/// // clean up
/// let clean = brown::remove_dir_brute("random_folder");
/// assert!(clean.is_ok());
/// ``` 
/// Incase you want to create a fersh folder every time you run your code, you can delete a folder using **remove_dir_brute** and then run **create_dir** with out the fear of an error.
pub fn create_dir( dir_path:&str)->Result<bool,Error> {
    let bp = BroPath::new();
    bp.sanitize(&dir_path.to_string())?;

    let complete = String::from("./") + &dir_path;
    let path = Path::new(&complete);
    //..................................
        match path.exists() {
            true=>{
                let e = Error::new(ErrorKind::AlreadyExists,"file already exists");
                return Err(e);
            } ,
            false=> {
                let d = fs::create_dir(path);
                match d {
                    Ok(()) => return Ok(true), // just changed 
                    Err(e) => Err(e),
                }
            },
        }
}