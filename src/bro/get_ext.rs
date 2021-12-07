use std::fs::{DirEntry};
use crate::BrownError;
/// The get_ext function will take a DirEntry object and return the
/// file extention. This saves us a lot of efforts and conversion
/// between types. 
pub fn get_ext(entry:&DirEntry)->Result<String,BrownError>{
    let path_buf = entry.path();
    let ext = path_buf.extension();
    match ext {
        Some(ft)=>{
            let ft_again = ft.to_str().map(|s| s.to_string());
                    match ft_again{
                     Some(ft_last)=>{
                        return Ok(ft_last);
                     },
                     None=>{
                        return Err(BrownError::FileExtError);   
                     },   
                    }
        },
        None=> {
            return Err(BrownError::FileExtError);
        },
    }
}