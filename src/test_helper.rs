use super::bro;
use crate::BrownError;

pub struct TestHelper {
pub parent_folder_name:&'static str,
}
impl TestHelper {
    pub fn new(parent_folder_name:&'static str)->Self{
        TestHelper {parent_folder_name}
    }
    pub fn setup_files_n_dirs(&self)->Result<bool,BrownError>{
      // parent folder
        let _aa = bro::create_dir(self.parent_folder_name)?;
      // 3 files
      bro::create_file_brute(format!("{}{}",self.parent_folder_name,"/fileA.md").as_str())?;
      bro::create_file_brute(format!("{}{}",self.parent_folder_name,"/fileB.md").as_str())?;
      bro::create_file_brute(format!("{}{}",self.parent_folder_name,"/fileC.md").as_str())?;
      
      // now 3 folders-- 
      bro::create_dir(format!("{}{}",self.parent_folder_name,"/dirA").as_str())?;
      bro::create_dir(format!("{}{}",self.parent_folder_name,"/dirB").as_str())?;
      bro::create_dir(format!("{}{}",self.parent_folder_name,"/dirC").as_str())?;
      
      Ok(true)
      
    }
    pub fn setup_dirs(&self)->Result<bool,BrownError>{
        // delete old
        let _= bro::remove_dir_brute(self.parent_folder_name);
        // parent folder
        let _aa = bro::create_dir(self.parent_folder_name)?;
      
      // level one - 1
      bro::create_dir(format!("{}/{}",
      self.parent_folder_name,"a").as_str())?;
      bro::create_dir(format!("{}/{}",
      self.parent_folder_name,"b").as_str())?;
      bro::create_dir(format!("{}/{}",
      self.parent_folder_name,"c").as_str())?;
      // level two - 2
      bro::create_dir(format!("{}/{}",
      self.parent_folder_name,"a/a1").as_str())?;
      bro::create_dir(format!("{}/{}",
      self.parent_folder_name,"a/a2").as_str())?;
      bro::create_dir(format!("{}/{}",
      self.parent_folder_name,"a/a3").as_str())?;
      
      bro::create_dir(format!("{}/{}",
      self.parent_folder_name,"b/b1").as_str())?;
      bro::create_dir(format!("{}/{}",
      self.parent_folder_name,"b/b2").as_str())?;
      bro::create_dir(format!("{}/{}",
      self.parent_folder_name,"b/b3").as_str())?;
      
      bro::create_dir(format!("{}/{}",
      self.parent_folder_name,"c/c1").as_str())?;
      bro::create_dir(format!("{}/{}",
      self.parent_folder_name,"c/c2").as_str())?;
      bro::create_dir(format!("{}/{}",
      self.parent_folder_name,"c/c3").as_str())?;
      
      //level three - 3
      bro::create_dir(format!("{}/{}",
      self.parent_folder_name,"a/a1/a11").as_str())?;
      bro::create_dir(format!("{}/{}",
      self.parent_folder_name,"b/b1/b11").as_str())?;
      bro::create_dir(format!("{}/{}",
      self.parent_folder_name,"c/c1/c11").as_str())?;
      Ok(true)
      
      }
    pub fn tear_down(&self)->Result<bool,BrownError>{
      // return type is same 
      bro::remove_dir_brute(self.parent_folder_name)
    }
      
}


