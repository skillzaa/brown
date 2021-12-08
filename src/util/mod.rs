use crate::BrownError;

pub fn vec_string_to_str(string_vec:&Vec<String>)
->Vec<&str>{
let mutated_str:Vec<&str>  = 
string_vec.iter().map(|s| &**s).collect();
mutated_str
}
pub fn vec_str_to_string(string_vec:&Vec<&str>)
->Vec<String>{
let mutated_str:Vec<String>  = 
string_vec.iter().map(|s| String::from(*s)).collect();
mutated_str
}

pub fn vec_replace_all(incomming:&Vec<String>,find:&str,replace_with:&str)->Result<Vec<String>,BrownError>{
    let mut output:Vec<String> = Vec::new();
    for s in incomming{
        let j = s.replace(find, replace_with);
        output.push(j);
    }
Ok(output)    
}
