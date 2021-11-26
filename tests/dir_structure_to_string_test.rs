use brown as bro;
use std::fs::DirEntry;
use std::io::{Error,ErrorKind};
use brown::tasks;
mod setup;
use setup::*;
#[cfg(test)]
#[test]
fn loop_test(){
let _ = setup::build_up_dirs();
let mut output:Vec<String> = Vec::new();
let level_one_de = 
bro::get_dirs(setup::PARENTFOLDER).unwrap();
let mut level_one_str = 
bro::direntry_to_path_all(&level_one_de, true)
.unwrap();
// get level one string into output
output.append(&mut level_one_str);
//=============================  
// This is from level 2 onwars (lvl 0 and 1 done)  
//=============================    
let mut seed = level_one_de;
loop {
let sub_dirs = 
bro::get_dirs_all(&seed).unwrap();   
if sub_dirs.len() < 1 {break;}

let mut paths = 
bro::direntry_to_path_all(&sub_dirs, true).unwrap();

output.append(&mut paths);
// for s in sub_dirs {
    
// }
// let mut seed:Vec<DirEntry> = Vec::new();
seed = sub_dirs;
}
//=============================    

let _ = setup::tear_down();
check_final(&output);
println!("{:#?}",output);
}

fn check_final(output:&Vec<String>)->bool {
let v: Vec<&str> = output.iter().map(|s| &**s).collect();
    assert_eq!(v,[
        "./tests/delme/b",
        "./tests/delme/a",
        "./tests/delme/c",
        "././tests/delme/a/a2",
        "././tests/delme/a/a1",
        "././tests/delme/a/a3",
        "././tests/delme/c/c1",
        "././tests/delme/c/c2",
        "./././tests/delme/c/c2/c22",
        "./././tests/delme/c/c2/c21",
    ]);
    true
}