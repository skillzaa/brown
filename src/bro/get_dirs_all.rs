use std::fs::DirEntry;
use crate::bro;
use crate::BrownError;
/// This fn takes a Vec of DirEntries and will return all the sub-directories of all the provided DirEntries.
/// It is used to get sub-folder of a directory at any given level by providing all the parent folders.
/// This function is helpful for traversing a directory structure.
pub fn get_dirs_all(incomming:&Vec<DirEntry>)->Result<Vec<DirEntry>,BrownError>{
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

#[cfg(test)]
mod tests {
use super::*;
use super::super::super::helper;
use super::bro::*;
#[test]
fn uno() {
let _ = helper::setup_dirs();
assert!(bro:: path_exists(helper::PARENTFOLDER));
let r = bro::get_dirs(helper::PARENTFOLDER);
assert!(r.is_ok());
let level_one = r.unwrap();
assert_eq!(level_one.len(),3);

//--------------------level-one
println!("Level One: {:?}",&level_one);
let paths = ["./delme/lvl1_a",
                    "./delme/lvl1_b",
                    "./delme/lvl1_c"];
for v in &level_one {
    let p = bro::direntry_to_path(&v).unwrap();
    assert!(paths.contains(&p.as_str()));
    // println!("{:?}",p);
}
//--------------------level-two
let level_two = 
bro::get_dirs_all(&level_one);
assert!(level_two.is_ok());
let level_two = level_two.unwrap();
    //println!("{:?}",level_two);

let paths = [
    "././delme/lvl1_a/lvl2_a1",
    "././delme/lvl1_a/lvl2_a2",
    "././delme/lvl1_a/lvl2_a3",                

    "././delme/lvl1_b/lvl2_b1",
    "././delme/lvl1_b/lvl2_b2",
    "././delme/lvl1_b/lvl2_b3",                

    "././delme/lvl1_c/lvl2_c1",
    "././delme/lvl1_c/lvl2_c2",
    "././delme/lvl1_c/lvl2_c3",                
                    ];

for v in &level_two {
    let p = 
    bro::direntry_to_path(v).unwrap();
    assert!(paths.contains(&p.as_str()));
}
//--------------------level-three
let level_three = 
bro::get_dirs_all(&level_two);
assert!(level_three.is_ok());
let level_three = level_three.unwrap();
    // println!("{:?}",&level_three);

let paths = [
    "./././delme/lvl1_a/lvl2_a1/lvl3_a11",
    "./././delme/lvl1_b/lvl2_b1/lvl3_b11",
    "./././delme/lvl1_c/lvl2_c1/lvl3_c11",
    ];

for v in &level_three {
    let p = 
    bro::direntry_to_path(v).unwrap();
    assert!(paths.contains(&p.as_str()));
}

let _= helper::tear_down();
assert!(!path_exists(helper::PARENTFOLDER));
}
} //tests mod ends