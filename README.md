# Brown
 > A simple library that makes dealing with Rust file system a breeze.
 ---

 Any one who has done any file system related project in Rust will know that the Rust file system (std::fs) is a bit wonky (so say the least).


 >**Brown** library exports helpful functions which gives you a very simple API for creating managin and manipulating files,folder etc in the current working folder.

 - It has zero dependencies.
 - **Keep in mind that thorugh out this library you do not need to add "./", it is added automatically**

 ---
 ## Example

```rust
use brown;

pub fn main(){
  // create a folder (parent folder) for rest of the tests
  let p_dir = brown::create_dir("parent");
  assert!(p_dir.is_ok());

  // create file safely
  let md_file = brown::create_file("parent/md_file.md");
  assert!(md_file.is_ok());
  
  // Will over write a file even if exists already.
  let html_file = brown::create_file_brute("parent/html_file.html");
  assert!(html_file.is_ok());
  
  // create a folder inside previously created parent folder 
  let test_folder = brown::create_dir("parent/test_folder");
  assert!(test_folder.is_ok());

  // This will just get the files from the given folder root.It returns a Vec of DirEntry objects (std::fs::DirEntry). Many fn in this lib consume DirEntry object.  
  let all_files = brown::get_files("parent");
  assert!(all_files.is_ok());
  
  // This will just get the folders from the given folder root. 
  let all_dirs = brown::get_dirs("parent");
  assert!(all_dirs.is_ok());
  
  // This will just get the files with .md extention from the given folder root.
  let all_md_files = brown::get_files_by_ext("parent","md");
  assert!(all_md_files.is_ok());
  // This will get all entries from a folder
  let all_entries = brown::get_entries("parent");
  assert!(all_entries.is_ok());

  }

```

 **Please note that there is no difference between a "folder" and "directory" in the documentation. The are both same.**
 **Do not add the ./ anywhere in the paths, this will be added automatically. There are no relative paths used.**

Incase you have any issues with the library, let me know here [https://github.com/skillzaa/brown/issues](https://github.com/skillzaa/brown/issues)
**Please be in touch**
My twitter handle is :: [rusthulk](https://twitter.com/rusthulk)

## Release Notes
[version 0.0.9](./notes/0_0_9.md)
[version 0.0.8](./notes/0_0_8.md)
