use std::io::{Error};
use crate::bro; //internallythe core of brown is known as bro

pub fn run()->Result<bool,Error>{
    let paths_list = vec!
            [ 
                "data", 
                "site" ,
                "site/images", 
                "hulkfolder" ,
                "hulkfolder/templates" ,
                "hulkfolder/config" ,
            ];
let parent_folder = "./cds/";
let mut nv:Vec<String> = Vec::new();

    for path in paths_list {
        let mut a = String::from(parent_folder);
        a.push_str(path);
        nv.push(a);
    }
            bro::create_dir("./cds")?;
            create_dir_structure(&nv)?;
Ok(true)                    
}

fn create_dir_structure(path_list:&Vec<String>)->Result<bool,Error>{


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