use std::fs;
use std::io::{Error};
use std::path::Path;

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

pub fn remove_file(file_path:&str)->Result<bool,Error>{
    let path = Path::new(file_path);
        let result  = fs::remove_file(&path);
        match result {
            Ok(()) => return Ok(true),
            Err(e) => return Err(e),
        }        
}


mod tests {
    use super::*;
    use super::super::*;
#[test]
fn basic(){
 let created = create_file_brute("some_file.md");
 assert!(created.is_ok());
 let deleted = remove_file("some_file.md");
 assert!(deleted.is_ok());   
}    
}
