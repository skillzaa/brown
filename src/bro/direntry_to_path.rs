use std::fs::DirEntry;
use crate::BrownError;
/// The direntry_to_path fn will take a Rust DirEntry type and return its path as string. This saves us 12 lines of code and 3 conversions.
pub fn direntry_to_path(direntry:&DirEntry)->Result<String,BrownError>{
    let path_buf = direntry.path();
    let f = path_buf.as_path().to_str().map(|s| s.to_string());
    match f {
        Some(ff)=> Ok(ff),
        None=>{
            Err(BrownError::DirEntryPathError)
        },
    }
    // let path = String::from(path_buf.as_path());
}
