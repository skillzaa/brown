use std::fs;
use std::io::{Error,ErrorKind};
use std::path::Path;

use crate::bro_path::BroPath;


pub fn create_dir_all( dir_path:&str)->Result<bool,Error> {
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
                let d = fs::create_dir_all(path);
                match d {
                    Ok(()) => return Ok(true), // just changed 
                    Err(e) => Err(e),
                }
            },
        }
}

#[cfg(test)]
mod tests {
use super::*;
#[test]    
    fn two(){
        let parent_created = 
        create_dir("parent/sub_folder/sub_sub_folder/sub_sub_sub_folder");
        println!("{:?}",parent_created);
        assert!(parent_created.is_ok());
        let parent_removed = remove_dir_brute("parent/sub_folder/sub_sub_folder/sub_sub_sub_folder");
        assert!(parent_removed.is_ok());
    
    }
    
}