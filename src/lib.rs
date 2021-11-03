use std::fs;
use std::fs::{File,DirEntry};
use std::path::Path;
use std::io::Write;
use std::io::{Error, ErrorKind};
pub trait Brown {
  fn create_file(&self,file_name:&str)->Result<File,Error>{
    // let file_name = "test_doc.txt";
    let my_file = File::create(file_name);
      match my_file {
        Ok(f) => return Ok(f),
        Err(e) => return Err(e),
      }
   } 
  fn delete_file(&self,file_name:&str)->Result<bool,Error>{
    let path = std::path::Path::new(file_name);
    // let path_exists = path.exists();
    match path.exists() {
      false => {
        let error = Error::new(ErrorKind::NotFound, "the path could not be found");
        return Err(error);
        },
      true => {
        let result  = fs::remove_file(&path);
        match result {
          // Ok(result) => return Ok(result),
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
  fn create_dir_all(&self,dir_name:String)->Result<bool,Error> {
    let full_path = String::from("./") + &dir_name;
    let path = std::path::Path::new(&full_path);
    let d = fs::create_dir_all(path);
    match d {
      Ok(()) => return Ok(true),
      Err(e) => return Err(e),
    }
  } 
  fn read_dir (&self,dir_name:&str)->Result<Vec<DirEntry>,Error>{
    let mut v:Vec<DirEntry> = Vec::new();
    for entry in fs::read_dir(dir_name).unwrap() {
      let entry = entry.unwrap();
      // let path = entry.path();  
              v.push(entry);
    }
    Ok(v)
  }
  fn get_dir_from_dir (&self,dir_name:&str)->Vec<DirEntry>{
    let mut v:Vec<DirEntry> = Vec::new();
        for entry in fs::read_dir(dir_name).unwrap() {
          let entry = entry.unwrap();
          let path = entry.path();
                if path.is_dir(){
                  v.push(entry);
                }
        }
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
  fn get_files_by_ext (&self,dir_name:&str,ext:&str)->Vec<DirEntry>{
    let mut v:Vec<DirEntry> = Vec::new();
        for entry in fs::read_dir(dir_name).unwrap() {
          let entry = entry.unwrap();
          let path = entry.path();
                if path.is_file(){
                  let e = path.extension().unwrap();
                  let e_str = e.to_str().unwrap();
                  if e_str == ext                    {
                    v.push(entry)
                  }
                }
        }
    v
  }
  fn path_exists(&self, value:&str)->bool{
    let path = std::path::Path::new(value);
    let tf = path.exists();
    tf
  }
}//final bracket
 
