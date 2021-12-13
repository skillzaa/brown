use crate::BrownError;
pub fn sanitize_file_path(input:&String)->Result<bool,BrownError>{
    let begin_with_alphabet =  qndr::begin_with_alphabet(input);
            match  begin_with_alphabet {
            false=>{
                return Err(BrownError::PathBeginWithAlphabet);
            },
            _=>{},
            }
    let alphanumeric_with_symbols  = 
    qndr::alphanumeric_with_symbols(input, "_-~/.");
    
    match  alphanumeric_with_symbols {
        false=>{
            return Err(BrownError::NonUrlSafeSymbolFound);
        },
        _=>{},
        }
    Ok(true)
}
pub fn sanitize_dir_path(input:&String)->Result<bool,BrownError>{
let begin_with_alphabet =  qndr::begin_with_alphabet(input);
        match  begin_with_alphabet {
        false=>{
            return Err(BrownError::PathBeginWithAlphabet);
        },
        _=>{},
        }
let alphanumeric_with_symbols  = 
qndr::alphanumeric_with_symbols(input, "_-~/");

match  alphanumeric_with_symbols {
    false=>{
        return Err(BrownError::NonUrlSafeSymbolFound);
    },
    _=>{},
    }
Ok(true)
}
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
