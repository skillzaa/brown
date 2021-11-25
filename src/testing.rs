use std::io::Error;

use super::bro;

pub static PARENTFOLDER: &str = "delme/";

pub fn setup_mixed()->Result<bool,Error>{
  // parent folder
  let _aa = bro::create_dir(PARENTFOLDER)?;

// 3 files
bro::create_file_brute(format!("{}{}",PARENTFOLDER,"/fileA.md").as_str())?;
bro::create_file_brute(format!("{}{}",PARENTFOLDER,"/fileB.md").as_str())?;
bro::create_file_brute(format!("{}{}",PARENTFOLDER,"/fileC.md").as_str())?;

// now 3 folders-- 
bro::create_dir(format!("{}{}",PARENTFOLDER,"/dirA").as_str())?;
bro::create_dir(format!("{}{}",PARENTFOLDER,"/dirB").as_str())?;
bro::create_dir(format!("{}{}",PARENTFOLDER,"/dirC").as_str())?;

Ok(true)

}
pub fn setup_dirs()->Result<bool,Error>{
  // delete old
  let _= bro::remove_dir_brute(PARENTFOLDER);
  // parent folder
  let _aa = bro::create_dir(PARENTFOLDER)?;

// level one - 1
bro::create_dir(format!("{}{}",
PARENTFOLDER,"lvl1_a").as_str())?;
bro::create_dir(format!("{}{}",
PARENTFOLDER,"lvl1_b").as_str())?;
bro::create_dir(format!("{}{}",
PARENTFOLDER,"lvl1_c").as_str())?;
// level two - 2
bro::create_dir(format!("{}{}",
PARENTFOLDER,"lvl1_a/lvl2_a1").as_str())?;
bro::create_dir(format!("{}{}",
PARENTFOLDER,"lvl1_a/lvl2_a2").as_str())?;
bro::create_dir(format!("{}{}",
PARENTFOLDER,"lvl1_a/lvl2_a3").as_str())?;

bro::create_dir(format!("{}{}",
PARENTFOLDER,"lvl1_b/lvl2_b1").as_str())?;
bro::create_dir(format!("{}{}",
PARENTFOLDER,"lvl1_b/lvl2_b2").as_str())?;
bro::create_dir(format!("{}{}",
PARENTFOLDER,"lvl1_b/lvl2_b3").as_str())?;

bro::create_dir(format!("{}{}",
PARENTFOLDER,"lvl1_c/lvl2_c1").as_str())?;
bro::create_dir(format!("{}{}",
PARENTFOLDER,"lvl1_c/lvl2_c2").as_str())?;
bro::create_dir(format!("{}{}",
PARENTFOLDER,"lvl1_c/lvl2_c3").as_str())?;

//level three - 3
bro::create_dir(format!("{}{}",
PARENTFOLDER,"lvl1_a/lvl2_a1/lvl3_a11").as_str())?;
bro::create_dir(format!("{}{}",
PARENTFOLDER,"lvl1_b/lvl2_b1/lvl3_b11").as_str())?;
bro::create_dir(format!("{}{}",
PARENTFOLDER,"lvl1_c/lvl2_c1/lvl3_c11").as_str())?;



Ok(true)

}
pub fn tear_down()->Result<bool,Error>{
// return type is same 
bro::remove_dir_brute(PARENTFOLDER)
}
