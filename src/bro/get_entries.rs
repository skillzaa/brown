use std::fs::DirEntry;
use crate::BrownError;
use crate::bro;

///The get_entries fn will get all the entries from a directory may it be files , folders or others. 
///If there is no entry in the said direcotry i.e there is no file or folder etc, in that case it will return
///an error. This will save the user from checking every time the returned vec if it has entries or not.
///It returns a Vec of DirEntry when successful. The DirEntry is a Rust struct used for holding any entry in a directory. 
///The dir_path should not have "./" since that will be added automatically.

pub fn get_entries(dir_path:&str)->Result<Vec<DirEntry>,BrownError>{
    
    let mut dir_entry_vec:Vec<DirEntry> = Vec::new();
    let read_dir = bro::get_read_dir(dir_path)?;
    for entry in read_dir {
        match entry {
            Ok(ent)=>{
                dir_entry_vec.push(ent);
            },
            Err(_e)=> continue,
        }
    }
    if dir_entry_vec.len() <= 0 {
        return Err(BrownError::DirEmpty);
    }else {
        return Ok(dir_entry_vec);
    } 
}