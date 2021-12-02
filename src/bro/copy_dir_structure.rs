// #[allow(dead_code)]
use std::io::{Error};
use crate::BroPath;
use crate::bro;

pub fn copy_dir_structure<'a>(source:&str,destination:&str)
->Result<Vec<String>,Error>{
//============= Step One    
let source_str = bro:: 
dir_structure_to_string(source)?;

//============ Step Two  
let bro_path = BroPath::new();
let paths_changed_parent = 
bro_path.change_destination(&source_str, source, destination)?;

let mutated = 
bro_path.remove_dot_slash_all(&paths_changed_parent)?;
//============ Step Three
let mutated_str:Vec<&str>  = 
mutated.iter().map(|s| &**s).collect();
let _x = bro::create_dir_structure
(&mutated_str);
   Ok(mutated) 
}
    