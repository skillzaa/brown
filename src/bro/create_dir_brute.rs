use std::fs;
use std::path::Path;
use crate::BrownError;
use crate::util;

/// The create_dir_brute fn is for times when you  want to create a directory without worring about if that directory already existed or not. If the folder already exists it will return true, if the folder does NOT exist it will create it and then return true. Error is returned only when the operating system call for creating folder fails.
/// This function is more concerned about creating the folder and less about saving the contents of the folder. This makes it a better candidate for creating setup folders etc.
/// If a path with non existing top level folders is given to create_dir_brute; those missing folders will also be created.  
pub fn create_dir_brute( dir_path:&str)->Result<bool,BrownError> {
   util::sanitize_dir_path(&dir_path.to_string())?;
        let complete = String::from("./") + &dir_path;
        let path = Path::new(&complete);
        //..................................
            match path.exists() {
                true=>{ return Ok(true)} ,
                false=> {
                    let d = fs::create_dir_all(path);
                    match d {
                        Ok(()) => return Ok(true), // just changed 
                        Err(_e) => Err(BrownError::FailedDirCreation),
                    }
                },
            }
    }
    
    #[cfg(test)]
    mod tests {
    use super::*;
    use super::super::*;
    use crate::BrownError;
    #[test]
    fn basic(){
        let _ = remove_dir_brute("dir683");
        
        let parent_created = create_dir_brute("dir683");
        assert!(parent_created.is_ok());
        let parent_removed = remove_dir_brute("dir683");
        assert!(parent_removed.is_ok());
    }
    #[test]
    fn no_dot_allowed(){
        
        let parent_created = 
        create_dir_brute("paren.t");
        assert!(parent_created.is_err());
        let res = parent_created.unwrap_err();
        assert_eq!(res,BrownError::NonUrlSafeSymbolFound);
    }
    #[test]
    fn no_dot_slash_allowed(){
        let parent_created = 
        create_dir_brute("./parent");
        assert!(parent_created.is_err());
        let res = parent_created.unwrap_err();
        assert_eq!(res,BrownError::PathBeginWithAlphabet);
    }
    }