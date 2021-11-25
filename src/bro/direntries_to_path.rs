use std::io::{Error,ErrorKind};
use std::fs::DirEntry;
use super::*;

/// This fn will take a Rust a Vec of &DirEntry type and return all of its paths as string.
/// If we set its "strict" flag to false then it will ignore the errors and return as many paths as it can.
/// Incase the "strict" flag is set to true then in will return error even if a single DirEntry fails to return path.
pub fn direntries_to_path(direntries:Vec<&DirEntry>,strict:bool)->Result<Vec<String>,Error>{
    let mut outcome:Vec<String> = Vec::new();
    for d in direntries {
        let p = 
        direntry_to_path(d);
            match  p {
                Ok(path)=>{
                    outcome.push(path);
                },
                Err(e)=>{
                    if strict == true {
                        return Err(e);
                    }else {
                        continue;
                    }
                },
            }
    }
    Ok(outcome)
}
