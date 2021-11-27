//=============// Allow //==================
/// The allow_numbers_only fn takes a &str and will check that it
/// should only consist of numerical chars.The fn will return 
/// false even if one non-numeric char is found.
pub fn allow_numbers_only(sample:&str )->bool{
    for c in sample.chars() { 
        let r = c.is_numeric();
        match r {
            true => {continue;},
            false=> return false,
        }
    }
true
}
/// The allow_alphanumeric_only fn takes a &str and check to see
/// that it should only consist of alpha-numerical chars.
/// The fn will return false even if one non-alpha-numerical char 
/// is found.    
pub fn allow_alphanumeric_only(sample:&str )->bool{
    for c in sample.chars() { 
        let r = c.is_alphanumeric();
        match r {
            true=> {},
            false=> return false,
        }
    }
true
}
/// The allow_alphabets_only fn takes a &str and check that it
/// should only consist of alphabets. It will return false even 
/// if one non-alphabetic char is found
pub fn allow_alphabets_only( sample:&str )->bool{
    for c in sample.chars() { 
        let r = c.is_alphabetic();
        match r {
            true=> {},
            false=> return false,
        }
    }
    true    
}

//=============// Get //==================
/// The get_alphabets fn takes a &str and return (as a new string) all
/// the alphabetic chars.
pub fn get_alphabets(sample:&str )->String{
    let mut alphabets = String::from(""); 
    for c in sample.chars() { 
        let r = c.is_alphabetic();
        match r {
            true=> {alphabets.push(c)},
            false=> {},
        }
    }
    alphabets    
}
/// The get_numbers fn takes a &str and return (as a new string) of 
/// all the numeric chars.
pub fn get_numbers(sample: &str)->String{
    let mut numbers = String::from(""); 
    for c in sample.chars() { 
        let r = c.is_numeric();
        match r {
            true=> {numbers.push(c)},
            false=> {},
        }
    }
    numbers    
}
/// The get_alphanumeric fn takes a &str and return (as a new string)
/// of all the alpha-numeric chars.
pub fn get_alphanumeric(sample:&str )->String{
    let mut alphanumeric = String::from(""); 
    for c in sample.chars() { 
        let r = c.is_alphanumeric();
        match r {
            true=> {alphanumeric.push(c)},
            false=> {},
        }
    }
    alphanumeric    
}
//=============// Get Non //==================    
/// The get_non_alphabets fn takes a &str and returns (as a new
/// string) all the **non alphabets** chars .
pub fn get_non_alphabets(sample:&str )->String{
    let mut non_alphabets = String::from(""); 
    for c in sample.chars() { 
        let r = !c.is_alphabetic(); //note !
        match r {
            true=> {non_alphabets.push(c)},
            false=> {},
        }
    }
    non_alphabets    
}
/// The get_non_numbers fn takes a &str and returns (as a new string)
/// of all the non-numeric chars .
pub fn get_non_numbers(sample:&str )->String{
    let mut numbers = String::from(""); 
    for c in sample.chars() { 
        let r = !c.is_numeric();
        match r {
            true=> {numbers.push(c)},
            false=> {},
        }
    }
    numbers    
}
/// The get_non_alphanumeric pub fn takes &str and returns (as a new
/// string) of all the non *Alpha-Numeric* chars.
pub fn get_non_alphanumeric(sample:&str )->String{
    let mut non_alphanumeric = String::from(""); 
    for c in sample.chars() { 
        let r = !c.is_alphanumeric();
        match r {
            true=> {non_alphanumeric.push(c)},
            false=> {},
        }
    }
    non_alphanumeric    
}

//=============// With Symbols //==================    
/// We use this pub fn if we want to allow just numbers and few
/// selected symbols e.g ! , @ etc. It takes a &str 
/// (the sample string) and another &str with allowed symbols.
/// It will return true only of the sample string has only
/// numbers and the allowed symbols provided.
pub fn numbers_with_symbols(sample:&str,allowed_symbols:&str)->bool{
    let just_numbers = get_numbers(&sample);
    let just_symbols = remove_chars(&sample , &just_numbers);
    let result = check_string_for_allowed_chars(&just_symbols,&allowed_symbols);
    result
}
/// The alphabets_with_symbols fn will allow just alphabets and the
/// selected symbols e.g ! , @ etc. 
/// It takes a &str (sample ) and another &str with allowed
/// symbols. It will return true only if the sample string has only
/// alphabets and the allowed symbols provided.

pub fn alphabets_with_symbols( sample:&str,allowed_symbols:&str )->bool{
    let non_alpha = get_non_alphabets(&sample);
    let result = check_string_for_allowed_chars
    (&non_alpha,&allowed_symbols);
    result
}
pub fn alphanumeric_with_symbols( sample:&str ,allowed_symbols:&str  )->bool{
    let non_alpha_num = get_non_alphanumeric(&sample);
    let result = check_string_for_allowed_chars
    (&non_alpha_num,&allowed_symbols);
    result
}  
//=============// Caps //==================    
/// The no_caps function will return None even if a single Capital
/// character is found in the provided sample &str.
/// *Any character other than a capital case alphabet is OK* 
pub fn no_caps( sample:&str )->bool{
    for i in sample.chars(){
        if i.is_ascii_uppercase() {return false;}
    }
    true
}
/// The data presented to this function should be **alphabatic**
/// and in all caps. If any char found that is not a **capital
/// alphabet** the function will return false.    
///The sample &str should be entirely comprising of 
/// *ABCDEFGHIJKLMNOPQRSTUVWXYZ*
pub fn only_caps(sample:&str)->bool{
    for i in sample.chars(){
        if !i.is_ascii_uppercase() {return false}
    }
    true
}

//=============// Begins and Ends //================== 

/// This function checks if the sample &str begins with the 
/// provided character or not. Return true if it does.
pub fn begin_with( sample:&str,begin_char:char )->bool{
    let first = sample.chars().nth(0);
    match first {
        Some(l)=>{
            if l == begin_char { 
                        return true
             } else 
                    { 
                        return false 
                    }
        },
        None=>{
            return false;
        }
    }
}
/// This function checks if the sample &str ends with the 
/// provided character or not. Return true if it does.

pub fn end_with( sample:&str,end_char:char )->bool{
    let last = sample.chars().rev().nth(0);
    match last {
        Some(l)=>{
            if l == end_char { 
                        return true
             } else 
                    { 
                        return false 
                    }
        },
        None=>{
            return false;
        }
    }
}
/// This function checks if the sample &str begins with a 
/// number or not. Return true if it does.

pub fn begin_with_number( sample:&str )->bool{
    let first = sample.chars().nth(0);
    match first {
        Some(l)=>{
            if l.is_numeric() { 
                        return true
             } else 
                    { 
                        return false 
                    }
        },
        None=>{
            return false;
        }
    }
}
/// This function checks if the sample &str ends with a 
/// number or not.

pub fn end_with_number( sample:&str )->bool{
    let last = sample.chars().rev().nth(0);
    match last {
        Some(l)=>{
            if l.is_numeric() { 
                        return true
             } else 
                    { 
                        return false 
                    }
        },
        None=>{
            return false;
        }
    } 
}
/// This function checks if the sample &str begins with an 
/// alphabet or not. Return true if it does.

pub fn begin_with_alphabet( sample:&str )->bool{
    let first = sample.chars().nth(0);
    match first {
        Some(l)=>{
            if l.is_alphabetic() { 
                        return true
             } else 
                    { 
                        return false 
                    }
        },
        None=>{
            return false;
        }
    } 
}

/// This function checks if the sample &str begins with an 
/// alphabet or not. Return true ifit does.
pub fn end_with_alphabet( sample:&str )->bool{
    let last = sample.chars().rev().nth(0);
    match last {
        Some(l)=>{
            if l.is_alphabetic() { 
                        return true
             } else 
                    { 
                        return false 
                    }
        },
        None=>{
            return false;
        }
    } 
}
/// The begin_with_alphanumeric function checks if the sample &str
/// begins with an alphanumeric or not.
pub fn begin_with_alphanumeric( sample:&str )->bool{
    let first = sample.chars().nth(0);
    match first {
        Some(l)=>{
            if l.is_alphanumeric() { 
                        return true
             } else 
                    { 
                        return false 
                    }
        },
        None=>{
            return false;
        }
    }
}
/// The end_with_alphanumeric function checks if the sample &str
/// ends with an alphanumeric or not. Return true if it does.
pub fn end_with_alphanumeric( sample:&str )->bool{
    let last= sample.chars().rev().nth(0);
    match last {
        Some(l)=>{
            if l.is_alphanumeric() { 
                        return true
             } else 
                    { 
                        return false 
                    }
        },
        None=>{
            return false;
        }
    }
}
    
//=============// Misc //================== 
/// This fn converts a &String into a Vec of chars
pub fn string_to_vec (incomming:&String)->Vec<char>{
    let mut chars:Vec<char> = Vec::new();
        for i in incomming.chars() {
            chars.push(i);
        }
    chars
}

/// This function will check the provided sample &str and check 
/// that it should **only** contain the provided characters. 
pub fn check_string_for_allowed_chars(data:&str,allowed_chars:&str)->bool{
    for i in data.chars(){
            match allowed_chars.contains(i){
                true=> {},
                false=> {return false}
            }
        }
    true
}
/// This function will remove all characters from the sample 
/// &str provided by the second argument.
pub fn remove_chars(sample:&str,subtract:&str)->String{
    let mut result:String = String::from("");
    for i in sample.chars(){
        if !subtract.contains(i){
            let j = i.clone();
            result.push(j); 
        }
    }
    result
}

   