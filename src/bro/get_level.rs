use std::fs::DirEntry;
use crate::bro;
use std::io::Error;
/// The get_level fn takes a Vec of DirEntries and will return all the sub-directories of all the provided DirEntries.
/// It is used to get all the sub-folder of a directory at any certain level by providing all the parent folders.
/// This function is helpful while traverting a directory structure.
pub fn get_level(incomming:&Vec<DirEntry>)->Result<Vec<DirEntry>,Error>{
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
    