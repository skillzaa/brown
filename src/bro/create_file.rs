use std::fs;
use std::fs::{ReadDir,DirEntry,File};
use std::io::{Error,ErrorKind};
use std::path::Path;
use crate::bro;
/// This function will create a file at given path as
/// long as the file does not exist already. In case 
/// the file aready exists it will return an error and
/// will not over write the file. 
/// Its operation is safe.
/// You need to give a complete file path : i.e file
///  path + file name + extention. 
/// However you do not need to add "./" before the path, 
/// that will be added automatically.
/// Example :: let x = hdir.create_file("parent_folder/child_folder/file_name.md");
/// In case any folder in the given path is not found it will an return error.

pub fn create_file(file_path:&str)->Result<File,Error>{
    let path_exist = bro::path_exists(file_path);
        match path_exist {
            true=>{
                let e = Error::new(ErrorKind::AlreadyExists,"file already exists");
                return Err(e);
            } ,
            false=> {
                let res = File::create(file_path);
                return res; // no need to Ok() it    
            },
        }
    } 