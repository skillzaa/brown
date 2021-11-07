use brown::Hdir;

#[cfg(test)]
#[test]
fn get_dir_test() {
    let hdir = Hdir::new().unwrap();
    let parent_path = "tests/kwh";
    // create parent folder
    let parent = hdir.create_dir(parent_path);
    assert!(parent.is_ok());
    
    ///////////-- add a file
    let c = hdir.create_file("tests/kwh/file_name.ext");
    assert!(c.is_ok());
    let d = hdir.create_file("tests/kwh/file_name2.ext");
    assert!(d.is_ok());
    let files_done = hdir.get_dirs(&parent_path);
    assert!(files_done.is_err()); // still no dirs

    // now create 2 dir ie a  , b 
    let a = hdir.create_dir("tests/kwh/a");
    assert!(a.is_ok());
    let b = hdir.create_dir("tests/kwh/b");
    assert!(b.is_ok());
    
    let dirs_done = hdir.get_dirs(&parent_path).unwrap();
    assert_eq!(dirs_done.len(),2); // just a and b are dirs

    
    //---now clean up=======del all folders
     //==== delete file
     let clean_c = hdir.remove_file("tests/kwh/file_name.ext");
     assert!(clean_c.is_ok());
     let clean_d = hdir.remove_file("tests/kwh/file_name2.ext");
     assert!(clean_d.is_ok());
    
     let files_clean = hdir.get_dirs(&parent_path).unwrap();
    assert_eq!(files_clean.len(),2); // still a and b are dirs

     //== noe del dirs
    let clean_a = hdir.remove_dir("tests/kwh/a"); 
    assert!(clean_a.is_ok()); // deleted dir
    let clean_b = hdir.remove_dir("tests/kwh/b"); 
    assert!(clean_b.is_ok()); // deleted dir
    
    let dirs_clean = hdir.get_files(&parent_path);
    assert!(dirs_clean.is_err()); // error since len = 0
    
   
    //====delete parent
    let parent_del = hdir.remove_dir(&parent_path);
    assert!(parent_del.is_ok());
}
