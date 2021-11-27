
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