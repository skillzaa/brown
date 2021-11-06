use std::fs;
use std::fs::{ReadDir,DirEntry};
use std::io::{Error,ErrorKind};
// mod core;
#[derive(Debug)]
pub struct Hdir {
  dir_name:String,  
}

impl Hdir{
    pub fn new(dir_name:String)->Hdir{
        let dir_name = "./".to_string() + dir_name.as_str();
        Hdir{
            dir_name,
        }
    }
    pub fn get_entries(&self)->Result<Vec<DirEntry>,Error>{
        let mut dir_entry_vec:Vec<DirEntry> = Vec::new();
        let read_dir = self.get_read_dir()?;
        for entry in read_dir {
            match entry {
                Ok(ent)=>{
                    dir_entry_vec.push(ent);
                },
                Err(e)=> continue,
            }
        }
     if dir_entry_vec.len() <= 0 {
         let e = Error::new(ErrorKind::NotFound,"found no valid entry in the directory");
         return Err(e);
     }else {
         return Ok(dir_entry_vec);
     } 
    }
    pub fn get_files(&self)->Result<Vec<DirEntry>,Error>{
        let mut vec:Vec<DirEntry> = Vec::new() ;
        let entries  = self.get_entries()?;
        for entry in entries {
            let is_file = is_file(&entry);
            match is_file {
                Ok(true) => {
                    vec.push(entry)
                },
                _=> continue,
            }
        }
        if vec.len() <= 0 {
            let e = Error::new(ErrorKind::NotFound,"found no files in the directory");
            return Err(e);
        }else {
            return Ok(vec);
        }
    }
    pub fn get_dirs(&self)->Result<Vec<DirEntry>,Error>{
        let mut vec:Vec<DirEntry> = Vec::new() ;
        let entries  = self.get_entries()?;
        for entry in entries {
            let is_dir = is_dir(&entry);
            match is_dir {
                Ok(true) => {
                    vec.push(entry)
                },
                _=> continue,
            }
        }
        if vec.len() <= 0 {
            let e = Error::new(ErrorKind::NotFound,"found no folders in the directory");
            return Err(e);
        }else {
            return Ok(vec);
        }
    }
    /// This is a wrapper function around rust fs::create_dir as per
    /// docs this is safe. It means that if the folder exists it will
    /// not be recreated.
    pub fn create(&self)->Result<bool,Error>{
        let path = std::path::Path::new(&self.dir_name);
        let d = fs::create_dir(path);
            match d {
                Ok(()) => return Ok(true),
                Err(e) => Err(e),
            }
    }    
    pub fn delete(&self)->Result<bool,Error>{
        let path = std::path::Path::new(&self.dir_name);
        let d = fs::remove_dir(path);
        match d {
            Ok(()) => return Ok(true),
            Err(e) => Err(e),
        }
    }    
    fn get_read_dir(&self)->Result<ReadDir,Error>{
        let read_dir = fs::read_dir(&self.dir_name);
        match read_dir {
            Ok(read_dir_ok)=>{
                return Ok(read_dir_ok);
            },
            Err(e)=>{
                let e = Error::new(ErrorKind::NotFound, "failed to read the directory, it may not exist or the path may be wrong");
                return Err(e);
            },
        }
    }
        // fn unwrap_direntry(&self,direntry:Result<DirEntry,Error>)->Result<DirEntry,Error>{
        // match direntry {
        //   Ok(direntry_final)=>{return Ok(direntry_final)},
        //   Err(e) => return Err(e),
        // }
        //}
    /////////////////////////////////////////    
/////////////////////////////////////////    
/////////////////////////////////////////    
/////////////////////////////////////////    
}////impl ends

fn path_exists( value:&str)->bool{
    let path = std::path::Path::new(value);
    let tf = path.exists();
    tf
  }
fn get_ext(entry:&DirEntry)->Result<String,Error>{
    let path_buf = entry.path();
    let ext = path_buf.extension();
    match ext {
        Some(ft)=>{
            let ft_again = ft.to_str().map(|s| s.to_string());
                    match ft_again{
                     Some(ft_last)=>{
                        return Ok(ft_last);
                     },
                     None=>{
                        let e = Error::new(ErrorKind::NotFound,"failed to get extention from file");
                        return Err(e);   
                     },   
                    }
        },
        None=> {
            let e = Error::new(ErrorKind::NotFound,"failed to get extention from file");
            return Err(e);
        },
    }
}
fn is_file(entry:&DirEntry)->Result<bool,Error>{
    let file_type = entry.file_type();
    match file_type {
        Ok(ft)=>{
            return Ok(ft.is_file());
        },
        Err(e)=> return Err(e),
    }
}
fn is_dir(entry:&DirEntry)->Result<bool,Error>{
    let file_type = entry.file_type();
    match file_type {
        Ok(ft)=>{
            return Ok(ft.is_dir());
        },
        Err(e)=> return Err(e),
    }
}