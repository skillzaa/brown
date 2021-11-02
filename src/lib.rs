use std::fs;
use std::fs::{File,DirEntry};
//use comrak::{markdown_to_html, ComrakOptions};
use std::path::Path;
use std::io::Write;

pub trait Brown {
  fn create_file(&self,file_name:&str)->File{
    // let file_name = "test_doc.txt";
    let my_file = File::create(file_name).expect("creation failed");
    my_file
  } 
  fn create_dir(&self,dir_name:&str)->std::io::Result<()> {
    let path = String::from("./") + &dir_name;
  
    let d = fs::create_dir(path)?;
    Ok(d)
  }
  fn create_dir_all(&self,dir_name:String)->std::io::Result<()> {
    //prepend the ./
    let path = String::from("./") + &dir_name;
    println!("path {}",path);
    let d = fs::create_dir_all(path)?;
    Ok(d)
  } 
  fn read_dir (&self,dir_name:&str)->Vec<DirEntry>{
    let mut v:Vec<DirEntry> = Vec::new();
    for entry in fs::read_dir(dir_name).unwrap() {
      let entry = entry.unwrap();
      // let path = entry.path();  
              v.push(entry);
    }
    v
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
 
