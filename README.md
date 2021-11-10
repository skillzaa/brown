# Brown
 > A simple library that makes dealing with Rust file system a breeze.
 ---

 Any one who has done any file system related project in Rust will know that the Rust file system (std::fs) is a bit wonky (so say the least).


 >**Brown** library exports helpful functions which gives you a very simple API for creating managin and manipulating files,folder etc in the current working folder.

 - It has zero dependencies.
 
 ---
 **For API documentation please check docs.rs/brown/0.0.6**
 ---
 ## Example

```rust
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
```

 **Please note that there is no difference between a "folder" and "directory". The are both same.**

Incase you have any issues with the library, let me know here [https://github.com/skillzaa/brown/issues](https://github.com/skillzaa/brown/issues)

My twitter handle is :: [rusthulk](https://twitter.com/rusthulk)



