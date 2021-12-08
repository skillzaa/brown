use crate::BrownError;
/// The create_dir_structure will take a &Vec of String (paths) ususally provided by dir_structure_to_string and create a folder structure using that.
pub fn create_dir_structure(paths_list:&Vec<&str>)
->Result<bool,BrownError>{
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
fn create_structure(path_list:&Vec<String>)->Result<bool,BrownError>{

for folder in path_list {
    let result = std::fs::create_dir_all(folder);
    match result {
        Ok(_f)=>{
            // a lib should not print ???
            //println!("folder created :: {}",folder)
        },
        Err(_e)=>{
            // println!("failed to create folder :: {} , the folder may already exist",folder);
            continue;
        },
    }
}
Ok(true)
}

mod tests {
use super::*;
use super::super::*;
#[test]
fn use_fn(){
    let paths_list = vec!
            [ 
                "/hulkfolder" ,
                "./data2", 
                "si?te" ,
                "data", 
                "data_with", 
                "site/images", 
                "hulkfolder/templates" ,
            ];
let a = create_dir_structure
(&paths_list);
assert!(a.is_ok());
    //====== tests
    assert_eq!(true,path_exists("./data"));
    assert_eq!(true,path_exists("./data_with"));
    assert_eq!(true,path_exists("./site/images"));
    assert_eq!(true,path_exists("./hulkfolder/templates")); 
// cleanup    
    // i can also just remove the base folders
    let data_removed = 
    remove_dir_brute("data");
        assert!(data_removed.is_ok());
    let _ = remove_dir_brute("data_with");
    let _ = remove_dir_brute("site/images");
    let _ = remove_dir_brute("./hulkfolder/templates"); 
    let _ = remove_dir_brute("./hulkfolder"); 
    let site_removed = 
    remove_dir_brute("site");
    assert!(site_removed.is_ok()); 
    }


}