// #[allow(dead_code)]
use std::io::{Error,ErrorKind};
use crate::qndr;


pub fn run(paths_list:&Vec<&str>)
->Result<bool,Error>{
let mut nv:Vec<String> = Vec::new();
    for path in paths_list {
    //----sanitize
        if !sanitize(path) {
            println!("path not correct: {}",path);
            continue;
        }
        let dir = format!("./{}",path);
        nv.push(dir);
    }
    create_structure(&nv)?;
Ok(true)                    
}
fn sanitize(input:&str)->bool{
let answer_one =  qndr::begin_with_alphabet(input);
let answer_two  = qndr::alphanumeric_with_symbols(input, "\\_-~/");
if answer_one && answer_two {true}else {false}
}
fn create_structure(path_list:&Vec<String>)->Result<bool,Error>{

for folder in path_list {
    let result = std::fs::create_dir_all(folder);
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