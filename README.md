# Brown
 > A simple library that makes dealing with Rust file system a breeze.
 ---

 Any one who has done any file system related project in Rust will know that the Rust file system (std::fs) is a bit wonky (so say the least).


 >**Brown** library exports struct "**Hdir**" (Hulk Directory). Once you create this struct it gives you a very simple API for creating managin and manipulating files,folder etc in the current working folder.


 - It has zero dependencies.
 
 ---
 ## Usage
```rust
use brown::Hdir;

fn main(){
        let hdir = Hdir::new().unwrap();

}
```
## The Api

1. **create_file** :: To create a file in the main folder or any of its sub-folders.
1. **remove_file** :: To delete a file from the main folder or any of its sub-folders.
1. **create_dir** :: To create a folder in the main folder or any of its sub-folders.
1. **remove_dir** :: To delete a folder in the main folder or any of its sub-folders.
1. **get_entries** :: Get all the entries of the main folder or any of its sub-folders. Entries include all the files, folder and sim links in a directory.
1. **get_files** :: Get all the files in the main folder or any of its sub-folders.
1. **get_files_by_ext** :: Get all the files with the given extention in the main folder or any of its sub-folders.
1. **get_read_dir** :: This will return an iterator over main folder or any of its sub-folders.

---


 **Please note that there is no difference between a "folder" and "directory". The are both same.**


The library is not mature by any standards... so try it and let me know.

Incase you have any issues with the library, let me know here [https://github.com/skillzaa/brown/issues](https://github.com/skillzaa/brown/issues)

My twitter handle is :: [rusthulk](https://twitter.com/rusthulk)



