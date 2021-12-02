// #[allow(dead_code)]
use std::io::{Error,ErrorKind};
use crate::bro;
pub fn run(source_folder:&str)->Result<Vec<String>,Error>{
let mut output:Vec<String> = Vec::new();
//===level zero =======
if !bro::path_exists(source_folder) {
    return Err(Error::new(ErrorKind::NotFound,"the source folder does not exist"));
}
output.push(String::from(source_folder));
//======lvl 0 ends=========

//======lvl 1 =========
let level_one_de = 
bro::get_dirs(source_folder)?;

let mut level_one_str = 
bro::direntry_to_path_all(&level_one_de, true)
?;
output.append(&mut level_one_str);
//=============================  
// This is from level 2 onwards (lvl 0 and 1 done)  
//=============================    
    let mut seed = level_one_de;
loop {
let sub_dirs = 
bro::get_dirs_all(&seed)?;   
if sub_dirs.len() < 1 {break;}

let mut paths = 
bro::direntry_to_path_all(&sub_dirs, true)
?;

output.append(&mut paths);
seed = sub_dirs;
}
   Ok(output) 
}
    