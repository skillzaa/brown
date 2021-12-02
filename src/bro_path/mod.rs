use std::io::{Error,ErrorKind};

/// This fn takes a &Vec<String> of paths and sanitize them one by one. The returned results are just the valid paths.
pub struct BroPath{}

impl BroPath{
    pub fn new()->Self{
        BroPath{}
    }
    pub fn sanitize_all(&self,paths:&Vec<String>)->Result<Vec<String>,Error>{
  
        let mut mutated:Vec<String> = Vec::new();
        for s in paths{
          if self.sanitize(s).is_ok() {
              mutated.push(s.to_owned())
          }else {
              continue;
          }
        }
    Ok(mutated)    
    }
    pub fn change_destination(&self,paths:&Vec<String>,source:&str,destination:&str)->Result<Vec<String>,Error>{
        let mut mutated:Vec<String> = Vec::new();
        for s in paths{
            let jj = s.replace(source, 
                destination);
            mutated.push(jj);
        }
    Ok(mutated)    
    }
    pub fn add_dot_slash(&self,path:&String)->String{
    format!("./{}",&path)
    }
    pub fn remove_dot_slash_all(&self,paths:&Vec<String>)->Result<Vec<String>,Error>{
        let mut mutated:Vec<String> = Vec::new();
        for s in paths{
            let j = s.replace("./", "");
            mutated.push(j);
        }
    Ok(mutated)    
    }
    /// The paths used through out this lib start with the name of the parent folder (obviously under the current working folder). There is no need to add "./" in the begining since that is done automatically.
    /// Path strings are allowed to have alphanumeric characters and only these symbols ("-" , "_" , "~" and "/" ). NO BACKSLASH ALLOWED
    /// The path should also not contain a "." 
    pub fn sanitize(&self,input:&String)->Result<bool,Error>{
        
    let begin_with_alphabet =  qndr::begin_with_alphabet(input);
            match  begin_with_alphabet {
            false=>{
                let e = Error::new(ErrorKind::InvalidInput,"path must begin with alphabet, remove ./ if any");
                return Err(e);
            },
            _=>{},
            }
    let alphanumeric_with_symbols  = 
    qndr::alphanumeric_with_symbols(input, "_-~/");
    
    match  alphanumeric_with_symbols {
        false=>{
            let e = Error::new(ErrorKind::InvalidInput,"you can only use url safe symbols (-,_,~)");
            return Err(e);
        },
        _=>{},
        }
    Ok(true)
    // if answer_one && answer_two {true}else {false}
    }
}





#[cfg(test)]
mod tests {
// use crate::util::vec_str_to_string;
use super::BroPath; 
#[test]
fn sanitize_path_test(){
let bro_path = BroPath::new();    
assert!(bro_path.sanitize(&"parent/sub1/sub2/sub3/sub4".to_string()).is_ok());
assert!(bro_path.sanitize(&"parent/sb_1/sb-2/sub~/sub4".to_string()).is_ok());
// -- this should not happen no dots between dirs
assert!(bro_path.sanitize(&"parent/sb.1/sb-2/sub~/sub4".to_string()).is_ok());
//-- this is ok
assert!(bro_path.sanitize(&"parent/su1/sub-2/sub~/sub4.html".to_string()).is_ok());
//============ NOT OK
assert!(bro_path.sanitize(&"par.ent/su.1/su.b-2/sub~/sub4.html".to_string()).is_err());

}

#[test]
fn sanitize_paths_test(){
let mut paths:Vec<String> = Vec::new();
paths.push(String::from("parent/sub1/sub2/sub3/sub4"));
paths.push(String::from("parent/sb_1/sb-2/sub~/sub4"));
paths.push(String::from("parent/sb.1/sb-2/sub~/sub4"));
paths.push(String::from("parent/su1/sub-2/sub~/sub4.html"));
let bp = BroPath::new();    
let x = bp.sanitize_all(&paths).unwrap();
assert!(x.len()==4);
// println!("{:#?}",x);
    }
}

