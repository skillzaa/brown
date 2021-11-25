#[allow(dead_code)]
use std::fs;
use std::fs::{ReadDir,DirEntry,File};
use std::io::{Error,ErrorKind};
use std::path::Path;
//------------------------
mod get_dirs_all; 
pub use get_dirs_all::*;
mod direntry_to_path; 
pub use direntry_to_path::*;
mod direntry_to_path_all; 
pub use direntry_to_path_all::*;
mod get_entries; 
pub use get_entries::*;
mod write_file;
pub use write_file::write_file;
mod get_files;
pub use get_files::get_files;
mod create_file;
pub use create_file::create_file;
mod create_dir;
pub use create_dir::create_dir;
mod get_read_dir;
pub use get_read_dir::get_read_dir;
mod get_ext;
pub use get_ext::get_ext;
mod get_files_by_ext;
pub use get_files_by_ext::get_files_by_ext;
mod get_dirs;
pub use get_dirs::get_dirs;
mod create_file_brute;
pub use create_file_brute::create_file_brute;
mod remove_dir;
pub use remove_dir::remove_dir;
mod is_dir;
pub use is_dir::is_dir;
//------------------------
/// The remove_file method will delete the file on the
/// given path.
/// It is a wrapper around fs::remove_file
pub fn remove_file(file_path:&str)->Result<bool,Error>{
    let path = Path::new(file_path);
        let result  = fs::remove_file(&path);
        match result {
            Ok(()) => return Ok(true),
            Err(e) => return Err(e),
        }        
}



/// The **remove_dir_brute** fn will delete a folder even if it has other files and folders. USE WITH CAUTION!!
/// Example :: ```
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
/// The get_file_name takes a DirEntry and return its file name with out the extentions.
pub fn get_file_name(dir_entry:&DirEntry)->Result<String,Error>{
    let e = Error::new(ErrorKind::NotFound, "could not extract file name (stem)");
    let path = dir_entry.path();
        match path.file_stem() {
            Some(file_stem_osstr)=>{
                let file_name_opt = file_stem_osstr.to_str().map(|s| s.to_string());
                    match file_name_opt {
                        Some(stem)=>{
                            Ok(stem)
                        },
                        None=>{
                            Err(e)
                        },
                    }
            },
            None=>{
                Err(e)
            },
        }
}  

/// The Rust std::fs::DirEntry path object has "exists" however
/// this function takes a &str and converts that into path 
/// thus is useful when the DirEntry object has not been obtained.
pub fn path_exists( value:&str)->bool{
    let path = Path::new(value);
    let tf = path.exists();
    tf
 }

pub fn is_file(entry:&DirEntry)->Result<bool,Error>{
    let file_type = entry.file_type();
    match file_type {
        Ok(ft)=>{
            return Ok(ft.is_file());
        },
        Err(e)=> return Err(e),
    }
}
