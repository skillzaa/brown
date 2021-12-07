use brown;
use brown::BrownError;
pub static PARENTFOLDER: &str = "tests/delme/";

/// Create 3 files  and 3 dirs 

pub fn build_up()->Result<bool,BrownError>{
  // parent folder
  let _aa = brown::create_dir(PARENTFOLDER)?;

// 3 files
brown::create_file_brute(format!("{}{}",PARENTFOLDER,"/fileA.md").as_str())?;
brown::create_file_brute(format!("{}{}",PARENTFOLDER,"/fileB.md").as_str())?;
brown::create_file_brute(format!("{}{}",PARENTFOLDER,"/fileC.md").as_str())?;

// now 3 folders-- 
brown::create_dir(format!("{}{}",PARENTFOLDER,"/dirA").as_str())?;
brown::create_dir(format!("{}{}",PARENTFOLDER,"/dirB").as_str())?;
brown::create_dir(format!("{}{}",PARENTFOLDER,"/dirC").as_str())?;

Ok(true)

}
pub fn build_up_dirs()->Result<bool,BrownError>{
  // delete old
  let _= brown::remove_dir_brute(PARENTFOLDER);
  // parent folder
  let _aa = brown::create_dir(PARENTFOLDER)?;

// now 3 folders-- 
brown::create_dir(format!("{}{}",PARENTFOLDER,"/a").as_str())?;
brown::create_dir(format!("{}{}",PARENTFOLDER,"/a/a1").as_str())?;
brown::create_dir(format!("{}{}",PARENTFOLDER,"/a/a2").as_str())?;
brown::create_dir(format!("{}{}",PARENTFOLDER,"/a/a3").as_str())?;
brown::create_dir(format!("{}{}",PARENTFOLDER,"/b").as_str())?;
brown::create_dir(format!("{}{}",PARENTFOLDER,"/c").as_str())?;
brown::create_dir(format!("{}{}",PARENTFOLDER,"/c/c1").as_str())?;
brown::create_dir(format!("{}{}",PARENTFOLDER,"/c/c2").as_str())?;
brown::create_dir(format!("{}{}",PARENTFOLDER,"/c/c2/c21").as_str())?;
brown::create_dir(format!("{}{}",PARENTFOLDER,"/c/c2/c22").as_str())?;

Ok(true)

}
pub fn tear_down()->Result<bool,BrownError>{
// return type is same 
brown::remove_dir_brute(PARENTFOLDER)
}
pub fn remove_files()->Result<bool,BrownError>{
brown::remove_file("tests/delme/fileA")?;
brown::remove_file("tests/delme/fileB")?;
brown::remove_file("tests/delme/fileC")?;
Ok(true)
}
pub fn remove_dirs()->Result<bool,BrownError>{
// create the mother folder
brown::remove_dir("tests/delme/dirA")?;
brown::remove_dir("tests/delme/dirB")?;
brown::remove_dir("tests/delme/dirC")?;
Ok(true)

}