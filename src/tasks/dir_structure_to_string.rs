// #[allow(dead_code)]
use std::io::{Error,ErrorKind};
use std::fs::DirEntry;
use crate::bro;

pub fn run(source_folder:&str)->Result<Vec<String>,Error>{
// // check the source folder exists, if not Error
// let _ =check_source_folder(source_folder)?;

// // place the source folder path in output
let mut output:Vec<String> = Vec::new();
// output.push(String::from(source_folder));

// // The Loop
//     // get into the loop (break on finished)
//     // - get dirs all
//     // - direntry to path_all
//     // - inser into out put
// // prep for loop    
// let mut finished = false;

// let mut level_de:Vec<DirEntry> = 
// bro::get_dirs(source_folder)?;

// let mut level_str = 
// bro::direntry_to_path_all(&level_de, true)?;

// let mut counter = 0;
// //=====LOOP====
// loop {

// if level_de.len() < 1 {finished = true;}
// if counter > 100 {finished = true;}
// if finished{break;}
// //--save new level_str
// let temp = bro::get_dirs_all(&level_de)?;

// //---get the new level_str into output
//  for n in level_str {
//     //  let p = bro::direntry_to_path(&n)?;
//      output.push(n);
//  }
//  //imp
// let level_str = temp;
//  counter = counter + 1;
// }
Ok(output)                    
}
// fn check_source_folder(source_folder:&str)->Result<bool,Error>{
//     if !bro::path_exists(&source_folder) {
//         let e = Error::new(ErrorKind::NotFound,"the source folder could not be found");
//         return Err(e);
//     }else {
//         Ok(true)
//     }
//}