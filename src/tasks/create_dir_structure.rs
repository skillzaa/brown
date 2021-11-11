use std::io::{Error,ErrorKind};
use crate::bro; //internallythe core of brown is known as bro
use qndr;

pub fn run(parent_folder:&str,paths_list:Vec<&str>)
->Result<bool,Error>{
let mut nv:Vec<String> = Vec::new();
    for path in paths_list {
    //----sanitize
        if !sanitize(path) {
            println!("path not correct: {}",path);
            continue;
        }
    
        let mut a = String::from(parent_folder);
        a.push_str(path);
        nv.push(a);
    }
    let parent_path = format!("./{}/", &parent_folder);
    bro::create_dir(parent_path.as_str())?;
    create_structure(&nv)?;
Ok(true)                    
}
fn sanitize(input:&str)->bool{
let answer_one =  qndr::begin_with_alphabet(input);
let answer_two  = qndr::alphabets_with_symbols(input, "\\/");
if answer_one && answer_two {true}else {false}
}
fn bool_to_result(input:bool,message:&str)->Result<bool,Error>{
    match input {
        true=>{
            return Ok(true);
        },
        false=>{
            let e = Error::new(ErrorKind::InvalidData,message);
            return Err(e);
        },
    }
}
fn create_structure(path_list:&Vec<String>)->Result<bool,Error>{

for folder in path_list {
    let result = bro::create_dir(folder);
    match result {
        Ok(_f)=>{println!("folder created :: {}",folder)},
        Err(_e)=>{
            println!("failed to create folder :: {} , the folder may already exist",folder);
            continue;
        },
    }
}
Ok(true)

}