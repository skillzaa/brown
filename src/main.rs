use std::io::Error;
use brown;

pub fn main(){
    // parent folder
    let _aa = brown::create_dir("parent");
  
  // files

  let _fileA = brown::create_file("parent/fileA.md");
  
  // folders-- 
  let _folderA = brown::create_dir("parent/dirA");
  
  let all_files = brown::get_files("parent");
  
  }