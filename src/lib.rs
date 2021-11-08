use std::fs;
use std::fs::{ReadDir,DirEntry,File};
use std::io::{Error,ErrorKind};
use std::path::Path;

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
    /// If there is no entry in the said direcotry i.e there is no 
    /// file or folder etc in it, in that case it will return
    /// an error. This will save the user from checking every time
    /// the returned vec if it has entries or not. 
    /// The dir_path should not have "./" and should not be 
    /// above the current working folder
    pub fn get_entries(&self,dir_path:&str)->Result<Vec<DirEntry>,Error>{
        let mut dir_entry_vec:Vec<DirEntry> = Vec::new();
        let read_dir = self.get_read_dir(dir_path)?;
        for entry in read_dir {
            match entry {
                Ok(ent)=>{
                    dir_entry_vec.push(ent);
                },
                Err(_e)=> continue,
            }
        }
     if dir_entry_vec.len() <= 0 {
         let e = Error::new(ErrorKind::NotFound,"found no valid entries in the directory");
         return Err(e);
     }else {
         return Ok(dir_entry_vec);
     } 
    }
    /// The get_files will get all the files from a folder leaving2
    /// out the directories.
    /// It will return error if no file is found thus the user does
    /// not have to check if the returned vec has some values or not. 
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
    /// The get_files_by_ext is just like get_files but it get files
    /// based on their extention.
    /// Do not add "." (the dot) with file extention
    /// Example::
    /// "md" , "html" , "txt" etc
    /// Do not include the "./" before the dir_path
    pub fn get_files_by_ext(&self,dir_path:&str,ext:&str)->Result<Vec<DirEntry>,Error>{
        let mut vec:Vec<DirEntry> = Vec::new() ;
        let files  = self.get_files(dir_path)?;
        for file in files {
            let file_ext = get_ext(&file)?;
                if file_ext == ext {
                        vec.push(file);
                }else {
                    continue;
                }
            
        }
        if vec.len() <= 0 {
            let e = Error::new(ErrorKind::NotFound,"found no files with the given extention");
            return Err(e);
        }else {
            return Ok(vec);
        }
    }
    /// The get_dirs will get all the directories from a directory
    /// leaving out the files.
    /// It will return error if no directory is found thus 
    /// the user does not have to check if the returned vec has 
    /// some values or not.  
    /// Do not include the "./" before the dir_path
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
    /// as the path is below its current working folder.
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
    /// In case any folder in the given path is not found it will return error.
    /// In case the file aready exists it will return error and will not over write the file. Its operation is safe.
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
    /// nature such that it can delete files anywhere in the 
    /// current folder    
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
    /// Keep in mind that though out this library the "./" is 
    /// added automatically 
    pub fn create_dir(&self, dir_name:&str)->Result<bool,Error> {
        let complete = String::from("./") + &dir_name;
        let path = std::path::Path::new(&complete);
        let d = fs::create_dir(path);
        match d {
          Ok(()) => return Ok(true),
          Err(e) => Err(e),
        }
      }
    pub fn remove_dir(&self, dir_name:&str)->Result<(),Error> {
        let complete = String::from("./") + &dir_name;
        let path = std::path::Path::new(&complete);
        let d = fs::remove_dir(path);
    d
    }
    // pub fn get_file_name(&self,path:&Path)->String{
    //     let file_name = path.file_name().unwrap();
    //     let file_name_str = file_name.to_str().map(|s| s.to_string()).unwrap();
    //     file_name_str
    // }  
    /// The get_read_dir will return "ReadDir" struct from Rust which
    /// is a iterator over the directory as per the path  
    fn get_read_dir(&self,dir_path:&str)->Result<ReadDir,Error>{
        let complete = String::from("./") + &dir_path;
        let path_com = std::path::Path::new(&complete);
        let read_dir = fs::read_dir(path_com);
        match read_dir {
            Ok(read_dir_ok)=>{
                return Ok(read_dir_ok);
            },
            Err(_e)=>{
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
pub fn get_ext(entry:&DirEntry)->Result<String,Error>{
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
pub fn is_file(entry:&DirEntry)->Result<bool,Error>{
    let file_type = entry.file_type();
    match file_type {
        Ok(ft)=>{
            return Ok(ft.is_file());
        },
        Err(e)=> return Err(e),
    }
}
pub fn is_dir(entry:&DirEntry)->Result<bool,Error>{
    let file_type = entry.file_type();
    match file_type {
        Ok(ft)=>{
            return Ok(ft.is_dir());
        },
        Err(e)=> return Err(e),
    }
}