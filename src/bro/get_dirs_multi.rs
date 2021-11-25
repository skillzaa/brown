use std::fs::DirEntry;
use crate::bro;
use std::io::Error;
/// The get_dirs_multi fn takes a Vec of DirEntries and will return all the sub-directories of all the provided DirEntries.
/// It is used to get all the sub-folder of a directory at any given level by providing all the parent folders.
/// This function is helpful for traversing a directory structure.
pub fn get_dirs_multi(incomming:&Vec<DirEntry>)->Result<Vec<DirEntry>,Error>{
let mut output:Vec<DirEntry> = Vec::new();    
for i in incomming {
    let res = 
    bro::direntry_to_path(&i);
        match res {
            Ok(path)=>{
                let sub_dirs = 
                bro::get_dirs(&path);
                match sub_dirs {
                    Ok(sdirs)=>{
                        for sd in sdirs {
                            // println!("sd{:?}",sd);
                            output.push(sd);
                        }
                    },
                    Err(_e)=>continue,    
                }
            },
            Err(_e)=>continue,
        }   
}
Ok(output)
}

#[cfg(test)]
mod tests {
use crate::{bro::testing::{setup_dirs, tear_down}, path_exists};

use super::*;
#[test]
fn uno() {
let _ = setup_dirs();
use super::bro::PARENTFOLDER;
// use super::bro::PARENTFOLDER;
assert!(path_exists(PARENTFOLDER));



let _ = tear_down();
}
} //tests mod ends