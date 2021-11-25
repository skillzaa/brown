use std::io::Error;
use super::*;

pub static PARENTFOLDER: &str = "delme/";

pub fn setup_mixed()->Result<bool,Error>{
  // parent folder
  let _aa = create_dir(PARENTFOLDER)?;

// 3 files
create_file_brute(format!("{}{}",PARENTFOLDER,"/fileA.md").as_str())?;
create_file_brute(format!("{}{}",PARENTFOLDER,"/fileB.md").as_str())?;
create_file_brute(format!("{}{}",PARENTFOLDER,"/fileC.md").as_str())?;

// now 3 folders-- 
create_dir(format!("{}{}",PARENTFOLDER,"/dirA").as_str())?;
create_dir(format!("{}{}",PARENTFOLDER,"/dirB").as_str())?;
create_dir(format!("{}{}",PARENTFOLDER,"/dirC").as_str())?;

Ok(true)

}
pub fn setup_dirs()->Result<bool,Error>{
  // delete old
  let _= remove_dir_brute(PARENTFOLDER);
  // parent folder
  let _aa = create_dir(PARENTFOLDER)?;

// now 3 folders-- 
create_dir(format!("{}{}",PARENTFOLDER,"/a").as_str())?;
create_dir(format!("{}{}",PARENTFOLDER,"/a/a1").as_str())?;
create_dir(format!("{}{}",PARENTFOLDER,"/a/a2").as_str())?;
create_dir(format!("{}{}",PARENTFOLDER,"/a/a3").as_str())?;
create_dir(format!("{}{}",PARENTFOLDER,"/b").as_str())?;
create_dir(format!("{}{}",PARENTFOLDER,"/c").as_str())?;
create_dir(format!("{}{}",PARENTFOLDER,"/c/c1").as_str())?;
create_dir(format!("{}{}",PARENTFOLDER,"/c/c2").as_str())?;
create_dir(format!("{}{}",PARENTFOLDER,"/c/c2/c21").as_str())?;
create_dir(format!("{}{}",PARENTFOLDER,"/c/c2/c22").as_str())?;

Ok(true)

}
pub fn tear_down()->Result<bool,Error>{
// return type is same 
remove_dir_brute(PARENTFOLDER)
}
