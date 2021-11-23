use std::io::{Error,ErrorKind};
use std::fs::{DirEntry};
use crate::bro; //internallythe core of brown is known as bro
// use qndr;

pub fn get_dir_structure(parent_dir_path:&str)->Result<Vec<String>,Error>{
//=========================   
todo!();
}

fn get_level(incomming:&Vec<DirEntry>)->Result<Vec<DirEntry>,Error>{
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
