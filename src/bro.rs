use std::fs;
use std::fs::{ReadDir,DirEntry,File};
use std::io::{Error,ErrorKind};
use std::path::Path;

///The get_entries fn will get all the entries from a directory may it be files , folders or others. 
///If there is no entry in the said direcotry i.e there is no file or folder etc, in that case it will return
///an error. This will save the user from checking every time the returned vec if it has entries or not.
///It returns a Vec of DirEntry when successful. The DirEntry is a Rust struct used for holding any entry in a directory. 
///The dir_path should not have "./" since that will be added automatically.

pub fn get_entries(dir_path:&str)->Result<Vec<DirEntry>,Error>{
    let mut dir_entry_vec:Vec<DirEntry> = Vec::new();
    let read_dir = get_read_dir(dir_path)?;
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
/// The get_files will get all the files from a folder leaving out the directories.
/// It will return error if no file is found thus the user does not have to check if the returned vec has some values or not. 
pub fn get_files(dir_path:&str)->Result<Vec<DirEntry>,Error>{
    let mut vec:Vec<DirEntry> = Vec::new() ;
    let entries  = get_entries(dir_path)?;
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
/// The get_files_by_ext is just like get_files but it get files based on their extention.
/// Do not add "." (the dot) with file extention
/// Example::
/// "md" , "html" , "txt" etc
/// Do not include the "./" before the dir_path
pub fn get_files_by_ext(dir_path:&str,ext:&str)->Result<Vec<DirEntry>,Error>{
    let mut vec:Vec<DirEntry> = Vec::new() ;
    let files  = get_files(dir_path)?;
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
/// The get_dirs will get all the directories from a 
/// directory leaving out the files.
/// It will return error if no directory is found thus 
/// the user does not have to check if the returned vec 
/// has some values or not.  
/// Note:: This function will dig in just one level. It will not look for sub-sub folders.
/// Do not include the "./" before the dir_path
pub fn get_dirs(dir_path:&str)->Result<Vec<DirEntry>,Error>{
    let mut vec:Vec<DirEntry> = Vec::new() ;
    let entries  = get_entries(dir_path)?;
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
/// The create_file_brute function is not safe. It means that it will
/// create a new file even if the old one exists. If this is not
/// what you want you should try create_file.
pub fn create_file_brute(file_path:&str)->Result<File,Error>{
    File::create(file_path)
    } 
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

/// The creation of a directory is always a safe 
/// operation i.e in case the folder already exists it 
/// will return an error.
/// Example :: ```
/// let p_dir = brown::create_dir("parent");
///  assert!(p_dir.is_ok());
/// ``` 
pub fn create_dir( dir_name:&str)->Result<bool,Error> {
    let complete = String::from("./") + &dir_name;
    let path = Path::new(&complete);
    //.................................................
        match path.exists() {
            true=>{
                let e = Error::new(ErrorKind::AlreadyExists,"file already exists");
                return Err(e);
            } ,
            false=> {
                let d = fs::create_dir(path);
                match d {
                    Ok(()) => return Ok(true), // just changed 
                    Err(e) => Err(e),
                }
            },
        }
}
/// The remove_dir funtion will remove a directory only of its empty.
/// Its operation is safe. This fn should be used normally unless brute removal is required 
/// Example :: ```
/// let p_dir = brown::create_dir("parent");
/// let outcome = brown::remove_dir("parent");
///  assert!(outcome.is_ok());
/// ``` 
pub fn remove_dir( dir_name:&str)->Result<bool,Error> {
    let complete = String::from("./") + &dir_name;
    let path = Path::new(&complete);
        match fs::remove_dir(path) {
            Ok(_ok)=>{
                Ok(true)
            },
            Err(e)=>{
                Err(e)
            },
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
/// The get_read_dir will return "ReadDir" struct from Rust which
/// is a iterator over the directory  
pub fn get_read_dir(dir_path:&str)->Result<ReadDir,Error>{
    let complete = String::from("./") + &dir_path;
    let path_com = Path::new(&complete);
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
/// The Rust std::fs::DirEntry path object has "exists" however
/// this function takes a &str and converts that into path 
/// thus is useful when the DirEntry object has not been obtained.
pub fn path_exists( value:&str)->bool{
    let path = Path::new(value);
    let tf = path.exists();
    tf
 }
/// The get_ext function will take a DirEntry object and return the
/// file extention. This saves us a lot of efforts and conversion
/// between types. 
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
            let e = Error::new(ErrorKind::NotFound,"failed to get extention");
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
/// We can directly get is_dir function using DirEntry. 
/// This saves us digging down two levels.
pub fn is_dir(entry:&DirEntry)->Result<bool,Error>{
    let file_type = entry.file_type();
    match file_type {
        Ok(ft)=>{
            return Ok(ft.is_dir());
        },
        Err(e)=> return Err(e),
    }
}
/// This function will create a file if it does not exist, and 
/// will entirely replace its contents if it does.
pub fn write_file(path:&str,content:&String)->Result<bool,Error>{
  fs::write(path, content)?;
  Ok(true)
}
pub fn direntry_to_path(direntry:&DirEntry)->Result<String,Error>{
    let path_buf = direntry.path();
    let f = path_buf.as_path().to_str().map(|s| s.to_string());
    match f {
        Some(ff)=> Ok(ff),
        None=>{
            let e = Error::new(ErrorKind::InvalidInput, "failed to extract path from Direntry");
            Err(e)
        },
    }
    // let path = String::from(path_buf.as_path());
}
// pub fn path_to_direntry(path_str:&str)->Result<DirEntry,Error>{
//     let path = Path::new(path_str);
//     let exists = path.exists();
//     match exists {
//         true=>{

//         },
//         false=>{
//             let e = Error::new(ErrorKind::InvalidInput, "failed to extract path from Direntry");
//             Err(e)
//         },
//     }
//     // let path = String::from(path_buf.as_path());
// }