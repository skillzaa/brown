use std::io::{Error,ErrorKind};
use std::fs::{DirEntry};
use crate::bro; //internallythe core of brown is known as bro
// use qndr;

pub fn get_dir_structure(parent_dir_path:&str)->Result<Vec<String>,Error>{

let mut output:Vec<String> = Vec::new();
let mut old :Vec<String> = get_level_paths
(parent_dir_path)?;

//=========================
let mut finished = true; 
loop {
    let mut new_data:Vec<String> = Vec::new();

    for o in &mut old {
        let sub_dir = 
        bro::get_dirs(&o);

        match sub_dir {
            Ok(sd)=>{
                for s in sd {
                    finished = false;
                    let p = 
                    bro::direntry_to_path(&s);
                        match p {
                            Ok(pp)=>{
                                new_data.push(pp);
                                output.push(o.to_string());
                            },
                            _=>{
                                output.push(o.to_string());
                            },
                        }
                }
            },
            Err(_e)=>{ continue;},
        }
        
    }
    //==========================================
    if new_data.len() < 1 { finished=true;}
    let old = new_data;
    //let new_data:Vec<String> = Vec::new();
    if finished==true{break;}
}    
//=========================   
Ok(output)    
}
fn get_level(parent_dir_path:&str)->Result<Vec<DirEntry>,Error>{
    let temp = bro::get_dirs(parent_dir_path);
    match temp {
        Ok(vec)=>{
            let mut output:Vec<DirEntry> = Vec::new() ;
                for v in vec {
                   output.push(v);     
                }
                Ok(output)
            },
            Err(e)=>return Err(e),
        }
}
fn get_level_paths(parent_dir_path:&str)->Result<Vec<String>,Error>{
    let temp = bro::get_dirs(parent_dir_path);
    match temp {
        Ok(vec)=>{
            let mut output:Vec<String> = Vec::new() ;
                for v in vec {
                    let p = 
                    bro::direntry_to_path(&v);
                    match p {
                        Ok(ppp)=>{
                            output.push(ppp);     
                        },
                        Err(e)=>return Err(e),
                    }
                }
                Ok(output)
            },
            Err(e)=>return Err(e),
        }
}

