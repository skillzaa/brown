use std::fs;
use std::fs::{ReadDir,DirEntry,File};
use std::io::{Error,ErrorKind};
// mod core;
#[derive(Debug)]
pub struct Hdir {
  current_dir:String,  
}

impl Hdir{
    pub fn new()->Result<Hdir,Error>{
        let c_d = std::env::current_dir();
        match c_d {
            Ok(c_d_ok)=>{
            let  current_dir= c_d_ok.as_path().to_str().map(|s| s.to_string());
                match current_dir {
                    Some(c)=>{
                        return Ok( Hdir{
                            current_dir:c,
                        });
                    },
                    None=>{
                        let e = Error::new(ErrorKind::NotFound,"the current working directory could not be assertained");
                        Err(e)
                    },        
                }
            },
            Err(e)=> {return Err(e)},
        }
    }
    /// get_entries will get all the entries from a directory may it
    /// be files , folders or others.
    /// If there is no entry in the said direcotry i.e there is no file or folder etc in the it, in that case it will also return an error. This will save the user from checking every time the returned vec if it has entries or not. 
    pub fn get_entries(&self,dir_path:&str)->Result<Vec<DirEntry>,Error>{
        let mut dir_entry_vec:Vec<DirEntry> = Vec::new();
        let read_dir = self.get_read_dir(dir_path)?;
        for entry in read_dir {
            match entry {
                Ok(ent)=>{
                    dir_entry_vec.push(ent);
                },
                Err(e)=> continue,
            }
        }
     if dir_entry_vec.len() <= 0 {
         let e = Error::new(ErrorKind::NotFound,"found no valid entries in the directory");
         return Err(e);
     }else {
         return Ok(dir_entry_vec);
     } 
    }
    pub fn get_files(&self,dir_path:&str)->Result<Vec<DirEntry>,Error>{
        let mut vec:Vec<DirEntry> = Vec::new() ;
        let entries  = self.get_entries(dir_path)?;
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
    pub fn get_dirs(&self,dir_path:&str)->Result<Vec<DirEntry>,Error>{
        let mut vec:Vec<DirEntry> = Vec::new() ;
        let entries  = self.get_entries(dir_path)?;
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
    /// This function will create a file at given path as long 
    /// as the path is below its running folder.
    /// You need to give a complete file path : i.e file path + file
    /// name + extention. 
    /// However you do not need to add "./" before the path, 
    /// that will be added automatically.
    /// ---
    /// Example:let x = hdir.create_file("first/second/file_name.md");
    /// ---
    /// The create_file (and also remove_file) is capable of 
    /// creating files anywhere in the current working folder and 
    /// its subfolders.
    pub fn create_file(&self,file_path:&str)->Result<File,Error>{
        let path_exist = path_exists(file_path);
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
    /// The remove_file method (just like create_file) are global in
    /// nature such that even if the Hdir object is created for a 
    /// sub-folder it can delete files in the current folder    
    pub fn remove_file(&self,file_path:&str)->Result<bool,Error>{
        let path = std::path::Path::new(file_path);
            let result  = fs::remove_file(&path);
            match result {
              Ok(()) => return Ok(true),
              Err(e) => return Err(e),
            }        
    }
    /// This is a wrapper function around rust fs::create_dir as per
    /// docs this is safe. It means that if the folder exists it will
    /// not be recreated.
    pub fn create_dir(&self, dir_name:&str)->Result<bool,Error> {
        let complete = String::from("./") + &dir_name;
        let path = std::path::Path::new(&complete);
        let d = fs::create_dir(path);
        match d {
          Ok(()) => return Ok(true),
          Err(e) => Err(e),
        }
      }
    // pub fn create_dir_all(dir_name:String)->Result<(),Error> {
    //     let full_path = String::from("./") + &dir_name;
    //     let path = std::path::Path::new(&full_path);
    //     let d = fs::create_dir_all(path);
    // d
    // }
    pub fn remove_dir(&self, dir_name:&str)->Result<(),Error> {
        let complete = String::from("./") + &dir_name;
        let path = std::path::Path::new(&complete);
        let d = fs::remove_dir(path);
    d
    }
    // pub fn remove_dir_all(dir_name:&str)->Result<(),Error> {
    //     let complete = String::from("./") + &dir_name;
    //     let path = std::path::Path::new(&complete);
    //     let d = fs::remove_dir_all(path);
    // d
    // }  
    fn get_read_dir(&self,dir_path:&str)->Result<ReadDir,Error>{
        let read_dir = fs::read_dir(dir_path);
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