use std::io::Error;
use super::bro;

pub struct TestSys {
parent_folder:&'static str,
}

impl TestSys {
    pub fn new ()->Self {
        TestSys { parent_folder: "delme/" }
    }
    pub fn setup_mixed(&self)->Result<bool,Error>{
    // parent folder
    let _aa = bro::create_dir(self.parent_folder)?;
    
    // 3 files
    bro::create_file_brute(format!("{}{}",self.parent_folder,"/fileA.md").as_str())?;
    bro::create_file_brute(format!("{}{}",self.parent_folder,"/fileB.md").as_str())?;
    bro::create_file_brute(format!("{}{}",self.parent_folder,"/fileC.md").as_str())?;
    
    // now 3 folders-- 
    bro::create_dir(format!("{}{}",self.parent_folder,"/dirA").as_str())?;
    bro::create_dir(format!("{}{}",self.parent_folder,"/dirB").as_str())?;
    bro::create_dir(format!("{}{}",self.parent_folder,"/dirC").as_str())?;
    
    Ok(true)
    
    }
    pub fn setup_dirs(&self)->Result<bool,Error>{
    // delete old
    let _= bro::remove_dir_brute(self.parent_folder);
    // parent folder
    let _aa = bro::create_dir(self.parent_folder)?;
    
    // level one - 1
    bro::create_dir(format!("{}{}",
    self.parent_folder,"lvl1_a").as_str())?;
    bro::create_dir(format!("{}{}",
    self.parent_folder,"lvl1_b").as_str())?;
    bro::create_dir(format!("{}{}",
    self.parent_folder,"lvl1_c").as_str())?;
    // level two - 2
    bro::create_dir(format!("{}{}",
    self.parent_folder,"lvl1_a/lvl2_a1").as_str())?;
    bro::create_dir(format!("{}{}",
    self.parent_folder,"lvl1_a/lvl2_a2").as_str())?;
    bro::create_dir(format!("{}{}",
    self.parent_folder,"lvl1_a/lvl2_a3").as_str())?;
    
    bro::create_dir(format!("{}{}",
    self.parent_folder,"lvl1_b/lvl2_b1").as_str())?;
    bro::create_dir(format!("{}{}",
    self.parent_folder,"lvl1_b/lvl2_b2").as_str())?;
    bro::create_dir(format!("{}{}",
    self.parent_folder,"lvl1_b/lvl2_b3").as_str())?;
    
    bro::create_dir(format!("{}{}",
    self.parent_folder,"lvl1_c/lvl2_c1").as_str())?;
    bro::create_dir(format!("{}{}",
    self.parent_folder,"lvl1_c/lvl2_c2").as_str())?;
    bro::create_dir(format!("{}{}",
    self.parent_folder,"lvl1_c/lvl2_c3").as_str())?;
    
    //level three - 3
    bro::create_dir(format!("{}{}",
    self.parent_folder,"lvl1_a/lvl2_a1/lvl3_a11").as_str())?;
    bro::create_dir(format!("{}{}",
    self.parent_folder,"lvl1_b/lvl2_b1/lvl3_b11").as_str())?;
    bro::create_dir(format!("{}{}",
    self.parent_folder,"lvl1_c/lvl2_c1/lvl3_c11").as_str())?;
    
    
    
    Ok(true)
    
    }
    pub fn tear_down(&self)->Result<bool,Error>{
    // return type is same 
    bro::remove_dir_brute(self.parent_folder)
    }
}
// pub static self.parent_folder: &str = "delme/";


