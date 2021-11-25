use std::fs;
use std::fs::{ReadDir,DirEntry,File};
use std::io::{Error,ErrorKind};
use std::path::Path;
use crate::bro;


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
