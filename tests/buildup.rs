use std::io::Error;
use brown;
pub static PARENTFOLDER: &str = "delme/";

/// Create 3 files  and 3 dirs 

pub fn build_up()->Result<bool,Error>{
  // parent folder
  let _aa = brown::create_dir("delme")?;

// 3 files
brown::create_file_brute(format!("{}{}",PARENTFOLDER,"/fileA.md").as_str())?;
brown::create_file_brute(format!("{}{}",PARENTFOLDER,"/fileB.md").as_str())?;
brown::create_file_brute(format!("{}{}",PARENTFOLDER,"/fileC.md").as_str())?;

// now 3 folders-- 
brown::create_dir(format!("{}{}",PARENTFOLDER,"/dirA.txt").as_str())?;
brown::create_dir(format!("{}{}",PARENTFOLDER,"/dirB.txt").as_str())?;
brown::create_dir(format!("{}{}",PARENTFOLDER,"/dirC.txt").as_str())?;

Ok(true)

}
pub fn tear_down()->Result<bool,Error>{
// return type is same 
brown::remove_dir_brute("delme")
}
pub fn remove_files()->Result<bool,Error>{
brown::remove_file("delme/fileA")?;
brown::remove_file("delme/fileB")?;
brown::remove_file("delme/fileC")?;
Ok(true)
}
pub fn remove_dirs()->Result<bool,Error>{
// create the mother folder
brown::remove_dir("delme/dirA")?;
brown::remove_dir("delme/dirB")?;
brown::remove_dir("delme/dirC")?;
Ok(true)

}