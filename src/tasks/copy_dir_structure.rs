// #[allow(dead_code)]
use std::io::{Error,ErrorKind};
use std::fs::DirEntry;
use crate::bro;
use crate::tasks;

pub fn run<'a>(source:&str,destination:&str)
->Result<Vec<String>,Error>{
//============= Step One    
    let mut source_str = 
     tasks::dir_structure_to_string
     ::run(source)?;
    println!("source_str :: {:#?}",source_str);

//============ Step Two    
let mut mutated:Vec<String> = Vec::new();
    for ss in &mut source_str{
        let jj = ss.replace(source, destination);
        let jjj = jj.replace("./", "");
        mutated.push(jjj);
    }

//============ Step Three
let mutated_str:Vec<&str>  = 
mutated.iter().map(|s| &**s).collect();

let x = tasks::create_dir_structure
::run(&mutated_str);
// Ok(true)    
   Ok(mutated) 
}
    