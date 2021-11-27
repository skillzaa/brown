// #[allow(dead_code)]
use std::io::{Error,ErrorKind};
use crate::tasks;
use crate::util;

pub fn run<'a>(source:&str,destination:&str)
->Result<Vec<String>,Error>{
//============= Step One    
    let source_str = 
     tasks::dir_structure_to_string
     ::run(source)?;
//============ Step Two  
let paths_changed_parent = 
util::paths_change_parent(&source_str, source, destination)?;

let mutated = 
util::paths_remove_dot_slash(&paths_changed_parent)?;
//============ Step Three
let mutated_str:Vec<&str>  = 
mutated.iter().map(|s| &**s).collect();
let _x = tasks::create_dir_structure
::run(&mutated_str);
// Ok(true)    
   Ok(mutated) 
}
    