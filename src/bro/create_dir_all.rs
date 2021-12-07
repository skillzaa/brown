use std::fs;
use std::io::{Error,ErrorKind};
use std::path::Path;

use crate::bro_path::BroPath;

/// The create_dir_all is just like brown::create_dir except it Recursively create a directory and all of its parent components if they are missing.
/// 
/// This is a simple wrapper function over Rust fs::create_dir_all.
/// 
/// The Operation of this function is also safe i.e It will not over-write an existing directory. 
/// 
/// # Example :
/// ```rust
///  // To ensure that old folder is cleared
/// let _ = brown::remove_dir_brute("parent");
/// let parent_created = 
/// brown::create_dir_all("parent/sub_folder/sub_sub_folder/sub_sub_sub_folder");       
/// assert!(parent_created.is_ok());
/// // cleanup
/// let parent_removed = 
/// brown::remove_dir_brute("parent/sub_folder/sub_sub_folder/sub_sub_sub_folder");
/// assert!(parent_removed.is_ok());
/// ```

pub fn create_dir_all(dir_path:&str)->Result<bool,Error> {
    let bp = BroPath::new();
    bp.sanitize(&dir_path.to_string())?;

    let complete = String::from("./") + &dir_path;
    let path = Path::new(&complete);
    //..................................
        match path.exists() {
            true=>{
                let e = Error::new(ErrorKind::AlreadyExists,"path already exists");
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
use super::super::*;
#[test]    
    fn create_n_del_three_sub_folders(){ 
        let _ = remove_dir_brute("parent");
        
        let parent_created = 
        create_dir_all("parent/sub_folder/sub_sub_folder/sub_sub_sub_folder");
        
        assert!(parent_created.is_ok());

        let parent_removed = 
        remove_dir_brute("parent");
        
        assert!(parent_removed.is_ok());
    
    }
#[test]    
    fn files_not_allowed(){
        
        let parent_created = 
        create_dir_all("parent/sub_folder/sub_sub_folder/sub_sub_sub_folder.html");

        assert!(parent_created.is_err());
    }
    
}