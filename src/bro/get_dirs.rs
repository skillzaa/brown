use crate::bro;
use crate::BrownError;
use std::fs::DirEntry;
/// The get_dirs will get all the directories from a 
/// directory leaving out the files.
/// It will return error if no directory is found thus 
/// the user does not have to check if the returned vec 
/// has some values or not.  
/// Note:: This function will dig in just one level. It will not look for sub-sub folders.
/// Do not include the "./" before the dir_path
pub fn get_dirs(dir_path:&str)->Result<Vec<DirEntry>,BrownError>{
    let mut vec:Vec<DirEntry> = Vec::new() ;
    let entries  = bro::get_entries(dir_path)?;
    for entry in entries {
        let is_dir = bro::is_dir(&entry);
        match is_dir {
            Ok(true) => {
                vec.push(entry)
            },
            _=> continue,
        }
    }
    if vec.len() <= 0 {
        return Err(BrownError::DirEmpty);
    }else {
        return Ok(vec);
    }
}

