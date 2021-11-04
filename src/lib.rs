use std::{fs};
use std::fs::{DirEntry, File, ReadDir};
use std::path::{Path};
use std::io::Write;
use std::io::{Error, ErrorKind};
pub trait Brown {
  fn create_file(&self,file_name:&str)->Result<File,Error>{
    let my_file = File::create(file_name);
      my_file
   } 
  fn delete_file(&self,file_name:&str)->Result<bool,Error>{
    let path = std::path::Path::new(file_name);
    match path.exists() {
      false => {
        let error = Error::new(ErrorKind::NotFound, "the path could not be found");
        return Err(error);
        },
      true => {
        let result  = fs::remove_file(&path);
        match result {
          Ok(()) => return Ok(true),
          Err(e) => return Err(e),
        }
      }  
    }
  } 
  fn create_dir(&self,dir_name:&str)->Result<bool,Error> {
    let complete = String::from("./") + &dir_name;
    let path = std::path::Path::new(&complete);
    let d = fs::create_dir(path);
    match d {
      Ok(()) => return Ok(true),
      Err(e) => Err(e),
    }
  }
  fn create_dir_all(&self,dir_name:String)->Result<(),Error> {
    let full_path = String::from("./") + &dir_name;
    let path = std::path::Path::new(&full_path);
    let d = fs::create_dir_all(path);
    d
  }
  fn remove_dir(&self,dir_name:&str)->Result<(),Error> {
    let complete = String::from("./") + &dir_name;
    let path = std::path::Path::new(&complete);
    let d = fs::remove_dir(path);
    d
  }
  fn remove_dir_all(&self,dir_name:&str)->Result<(),Error> {
    let complete = String::from("./") + &dir_name;
    let path = std::path::Path::new(&complete);
    let d = fs::remove_dir_all(path);
    d
  } 
  fn read_dir (&self,dir_name:&str)->Result<ReadDir,Error>{
    let dir_entry = fs::read_dir(dir_name).expect("failed to read directory");
    Ok(dir_entry)
  }
  fn get_dir_from_dir (&self,dir_name:&str)->Vec<DirEntry>{
    let  all = self.read_dir(dir_name).unwrap();
    let v = 
    all.map(|x|x.unwrap())
    .filter(|x| (&x.path()).is_dir()).collect();
    v
  }
  fn get_files_from_dir (&self,dir_name:&str)->Vec<DirEntry>{
    let mut v:Vec<DirEntry> = Vec::new();
        for entry in fs::read_dir(dir_name).unwrap() {
          let entry = entry.unwrap();
          let path = entry.path();
                if path.is_file(){
                  v.push(entry);
                }
        }
    v
  }
  fn check_n_create_folders (&self,folders_paths_list:Vec<&str>)->Option<bool>{
    for item in folders_paths_list {
        let data_folder_path = Path::new(item);
        let data_folder_exists = data_folder_path.exists();
        if !data_folder_exists {
            self.create_dir(item).unwrap();
            println!("folder created:: {}",item.to_string());
        }else {
            println!("folder aleady exists:: {}",item.to_string());            
        }
    }
    Some(true)
}
  fn write_to_file(&self,file_name:&str,content:&str) -> std::io::Result<()> {
  let mut f = std::fs::OpenOptions::new().write(true).open(file_name)?;
  f.write(content.as_bytes())?;
  f.flush()?;
  Ok(())
  }
  fn get_files_by_ext (&self,dir_name:&str,ext:&str)
  ->Result<Vec<DirEntry>,Error>{
    let all_files = fs::read_dir(dir_name).expect("failed to open dorectory");
    let mut v:Vec<DirEntry> = Vec::new();
    for entry in all_files {
        let entry = entry.expect("failed to open directory entry");
          let path_buf = entry.path();
            let buf_option = path_buf.extension().unwrap().to_str();
              let buf_ext = buf_option.unwrap();
                  if buf_ext == ext{
                        v.push(entry);
                  }

    }
    Ok(v)
  }
  fn path_exists(&self, value:&str)->bool{
    let path = std::path::Path::new(value);
    let tf = path.exists();
    tf
  }
}//final bracket
 
