#[allow(dead_code)]
use std::fs;
use std::fs::{ReadDir,DirEntry,File};
use std::io::{Error,ErrorKind};
use std::path::Path;
//------------------------
mod get_dirs_all; 
pub use get_dirs_all::*;
mod direntry_to_path; 
pub use direntry_to_path::*;
mod direntry_to_path_all; 
pub use direntry_to_path_all::*;
mod get_entries; 
pub use get_entries::*;
mod write_file;
pub use write_file::write_file;
mod get_files;
pub use get_files::get_files;
mod create_file;
pub use create_file::create_file;
mod create_dir;
pub use create_dir::create_dir;
mod get_read_dir;
pub use get_read_dir::get_read_dir;
mod get_ext;
pub use get_ext::get_ext;
mod get_files_by_ext;
pub use get_files_by_ext::get_files_by_ext;
mod get_dirs;
pub use get_dirs::get_dirs;
mod create_file_brute;
pub use create_file_brute::create_file_brute;
mod remove_dir;
pub use remove_dir::remove_dir;
mod is_dir;
pub use is_dir::is_dir;
mod remove_file;
pub use remove_file::remove_file;
mod remove_dir_brute;
pub use remove_dir_brute::remove_dir_brute;
mod get_file_name;
pub use get_file_name::get_file_name;
mod path_exists;
pub use path_exists::path_exists;
mod is_file;
pub use is_file::is_file;
//------------------------

