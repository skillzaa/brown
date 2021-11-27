use std::fs;
use std::io::{Error,ErrorKind};
use std::path::Path;
// use crate::bro;
use crate::util;
/// The creation of a directory is always a safe 
/// operation i.e in case the folder already exists it 
/// will return an error.
/// Example :: ```
/// let p_dir = brown::create_dir("parent");
///  assert!(p_dir.is_ok());
/// ``` 
pub fn create_dir( dir_name:&str)->Result<bool,Error> {
    // util::
    let complete = String::from("./") + &dir_name;
    let path = Path::new(&complete);
    //.................................................
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