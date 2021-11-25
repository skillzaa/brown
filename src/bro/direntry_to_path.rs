use std::io::{Error,ErrorKind};
use std::fs::DirEntry;
/// This fn will take a Rust DirEntry type and return its path as string. This saved us 12 lines of code and 3 conversions.
pub fn direntry_to_path(direntry:&DirEntry)->Result<String,Error>{
    let path_buf = direntry.path();
    let f = path_buf.as_path().to_str().map(|s| s.to_string());
    match f {
        Some(ff)=> Ok(ff),
        None=>{
            let e = Error::new(ErrorKind::InvalidInput, "failed to extract path from Direntry");
            Err(e)
        },
    }
    // let path = String::from(path_buf.as_path());
}
