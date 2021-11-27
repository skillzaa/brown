use std::io::{Error,ErrorKind};

/// This fn takes a &Vec<String> of paths and sanitize them one by one. The returned results are just the valid paths.
pub fn paths_sanitize(paths:&Vec<String>)->Result<Vec<String>,Error>{
    let mut mutated:Vec<String> = Vec::new();
    for s in paths{
      if path_sanitize(s){
          mutated.push(s.to_owned())
      }else {
          continue;
      }
    }
Ok(mutated)    
}
pub fn paths_change_parent(paths:&Vec<String>,source:&str,destination:&str)->Result<Vec<String>,Error>{
    let mut mutated:Vec<String> = Vec::new();
    for s in paths{
        let jj = s.replace(source, 
            destination);
        mutated.push(jj);
    }
Ok(mutated)    
}
pub fn paths_remove_dot_slash(paths:&Vec<String>)->Result<Vec<String>,Error>{
    let mut mutated:Vec<String> = Vec::new();
    for s in paths{
        let j = s.replace("./", "");
        mutated.push(j);
    }
Ok(mutated)    
}
/// The paths used through out this lib start with the name of the parent folder (obviously under the current working folder). There is no need to add "./" in the begining since that is done automatically.
/// Path strings are allowed to have alphanumeric characters and only these symbols ("-" , "_" , "~" , "\" and "/" )
/// The path of a directory should not contain a "." only file path can contain a "." 
pub fn path_sanitize(input:&str)->bool{
let answer_one =  qndr::begin_with_alphabet(input);
let answer_two  = qndr::alphanumeric_with_symbols(input, "\\_-~./");
if answer_one && answer_two {true}else {false}
}

#[cfg(test)]
mod tests {
 use crate::util::vec_str_to_string;

use super::*; 
 #[test]
 fn sanitize_path_test(){
    assert!(path_sanitize("parent/sub1/sub2/sub3/sub4"));
    assert!(path_sanitize("parent/sb_1/sb-2/sub~/sub4"));
    // -- this should not happen no dots between dirs
    assert!(path_sanitize("parent/sb.1/sb-2/sub~/sub4"));
    //-- this is ok
    assert!(path_sanitize("parent/su1/sub-2/sub~/sub4.html"));

 }
 #[test]
 fn sanitize_paths_test(){
let mut paths:Vec<String> = Vec::new();
paths.push(String::from("parent/sub1/sub2/sub3/sub4"));
paths.push(String::from("parent/sb_1/sb-2/sub~/sub4"));
paths.push(String::from("parent/sb.1/sb-2/sub~/sub4"));
paths.push(String::from("parent/su1/sub-2/sub~/sub4.html"));
    
let x = paths_sanitize(&paths).unwrap();
assert!(x.len()==4);
// println!("{:#?}",x);

 }
 

}