use std::fs;
use std::path::Path;
use crate::BrownError;
/// The remove_file method will delete the file on the
/// given path.
/// It is a wrapper around fs::remove_file
/// 
/// # Example
/// ```rust
///  // Make sure the file exists
///  let created = brown::create_file_brute("some_file.md");
/// assert!(created.is_ok());
/// let deleted = brown::remove_file("some_file.md");
/// assert!(deleted.is_ok());   
/// ```

pub fn remove_file(file_path:&str)->Result<bool,BrownError>{
    let path = Path::new(file_path);
        let result  = fs::remove_file(&path);
        match result {
            Ok(()) => return Ok(true),
            Err(_e) => return Err(BrownError::FailedFileDeletion),
        }        
}


mod tests {
    use super::super::*;
#[test]
fn basic(){
 let created = create_file_brute("some_file.md");
 assert!(created.is_ok());
 let deleted = remove_file("some_file.md");
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
}
