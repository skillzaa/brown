use brown;

pub fn main(){
  // create a folder (parent folder) for rest of the tests
  let p_dir = brown::create_dir("parent");
  assert!(p_dir.is_ok());

  // create file safely
  let md_file = brown::create_file("parent/md_file.md");
  assert!(md_file.is_ok());
  
  // Will over write a file even if exists already.
  let html_file = brown::create_file_brute("parent/html_file.html");
  assert!(html_file.is_ok());
  
  // create a folder inside previously created parent folder 
  let test_folder = brown::create_dir("parent/test_folder");
  assert!(test_folder.is_ok());

  // This will just get the files from the given folder root.It returns a Vec of DirEntry objects (std::fs::DirEntry). Many fn in this lib consume DirEntry object.  
  let all_files = brown::get_files("parent");
  assert!(all_files.is_ok());
  
  // This will just get the folders from the given folder root. 
  let all_dirs = brown::get_dirs("parent");
  assert!(all_dirs.is_ok());
  
  // This will just get the files with .md extention from the given folder root.
  let all_md_files = brown::get_files_by_ext("parent","md");
  assert!(all_md_files.is_ok());
  // This will get all entries from a folder
  let all_entries = brown::get_entries("parent");
  assert!(all_entries.is_ok());

  }