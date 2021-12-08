use crate::BrownError;
/// The create_dir_structure will take a &Vec of String (paths) ususally provided by brown::dir_structure_to_string and create a folder structure using that.
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
        Ok(_f)=>{println!("folder created :: {}",folder)},
        Err(_e)=>{
            // println!("failed to create folder :: {} , the folder may already exist",folder);
            continue;
        },
    }
}
Ok(true)

}