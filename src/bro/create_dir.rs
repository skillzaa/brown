use std::fs;
use std::path::Path;
use crate::BrownError;
use crate::util;

// use crate::bro_path::BroPath;
/// The create_dir function will create a folder safely i.e in case the folder does not exist, it will be created But incase the folder already exists the function will return an error.
/// 
/// If you do not care that the folder already exists or not and you want to create a fersh folder every time you run your code, you should delete any older folder using **remove_dir_brute** first and then run **create_dir** to remove any fear of error being returned OR you can use create_dir_brute.

/// Do not use **./** in the dir_path argument, just give the fully qualified path.
/// 
/// This function is a simple wrapper around Rust fs::create_dir function to integrate the rust function well with brown library. Also [check](https://doc.rust-lang.org/std/fs/fn.create_dir.html) 
/// 
/// # Example (If folder does not exist)
/// ```rust
/// // To ensure that previously created folder is removed
/// let _ = brown::remove_dir_brute("random_folder");
/// let p_dir = brown::create_dir("random_folder");
///  assert!(p_dir.is_ok());
/// // clean up 
/// let clean = brown::remove_dir_brute("random_folder");
/// assert!(clean.is_ok());
/// ```
///  
/// # Example (If folder Already exists)
/// ```rust
/// // To ensure that previously created folder is removed
/// let _ = brown::remove_dir_brute("rand_folder");
/// // first time creation
/// let p_dir = brown::create_dir("rand_folder");
///  assert!(p_dir.is_ok());
/// // Second time creation
/// let p_dir_again = brown::create_dir("rand_folder");
///  assert!(p_dir_again.is_err());
/// // clean up
/// let clean = brown::remove_dir_brute("rand_folder");
/// assert!(clean.is_ok());
/// ``` 
/// # Example (No dot allowed in path)
/// ```rust
/// let parent_created = 
/// brown::create_dir("paren.t");
/// assert!(parent_created.is_err());
/// ```
/// # Example (No dot slash allowed in path start)
/// ```rust
/// let parent_created = 
/// brown::create_dir("./parent");
/// assert!(parent_created.is_err());
/// ```
/// # Example (Only symbols allowed in path are "-,_,~,/")
/// ```rust
/// let parent_created = 
/// brown::create_dir("p-a_ren~t");
/// assert!(parent_created.is_ok());
/// // cleanup
/// let _ = brown::remove_dir_brute("p-a_ren~t");
/// ```
/// # Example (You can create a sub-folder only if the parent has been created already)
/// ```rust
/// let parent_created = 
/// brown::create_dir("parent/sub_folder");
/// assert!(parent_created.is_err());
/// ```
/// # Example (Creating sub-folders)
/// ```rust
/// let _ = brown::remove_dir_brute("folder_no_234");
/// let parent_created = 
/// brown::create_dir("folder_no_234");
/// assert!(parent_created.is_ok());
/// let sub01_created = 
/// brown::create_dir("folder_no_234/sub01");
/// let sub02_created = 
/// brown::create_dir("folder_no_234/sub02");
/// assert!(sub02_created.is_ok());
/// // clean up
/// let _ = brown::remove_dir_brute("folder_no_234");
/// ```

pub fn create_dir( dir_path:&str)->Result<bool,BrownError> {
   
util::sanitize_dir_path(&dir_path.to_string())?;
    let complete = String::from("./") + &dir_path;
    let path = Path::new(&complete);
    //..................................
        match path.exists() {
            true=>{
                return Err(BrownError::FileAlreadyExists);
            } ,
            false=> {
                let d = fs::create_dir(path);
                match d {
                    Ok(()) => return Ok(true), // just changed 
                    Err(e) => Err(BrownError::FailedDirCreation),
                }
            },
        }
}

#[cfg(test)]
mod tests {
use super::*;
use super::super::*;
#[test]
fn basic(){
    let _ = remove_dir_brute("create_dir_folder");
    
    let parent_created = create_dir("create_dir_folder");
    assert!(parent_created.is_ok());
    let parent_removed = remove_dir_brute("create_dir_folder");
    assert!(parent_removed.is_ok());
}
#[test]
fn no_dot_allowed(){
    
    let parent_created = 
    create_dir("paren.t");
    assert!(parent_created.is_err());
}
#[test]
fn no_dot_slash_allowed(){
    let parent_created = 
    create_dir("./parent");
    assert!(parent_created.is_err());
}
}