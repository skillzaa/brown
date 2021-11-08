use brown;

#[cfg(test)]
#[test]
fn get_dir_test() {
    let parent_path = "tests/kwh";
    // create parent folder
    let parent = brown::create_dir_safe(parent_path);
    assert!(parent.is_ok());
    
    ///////////-- add a file
    let c = brown::create_file_safe("tests/kwh/file_name.ext");
    assert!(c.is_ok());
    let d = brown::create_file_safe("tests/kwh/file_name2.ext");
    assert!(d.is_ok());
    let files_done = brown::get_dirs(&parent_path);
    assert!(files_done.is_err()); // still no dirs

    // now create 2 dir ie a  , b 
    let a = brown::create_dir_safe("tests/kwh/a");
    assert!(a.is_ok());
    let b = brown::create_dir_safe("tests/kwh/b");
    assert!(b.is_ok());
    
    let dirs_done = brown::get_dirs(&parent_path).unwrap();
    assert_eq!(dirs_done.len(),2); // just a and b are dirs

    
    //---now clean up=======del all folders
     //==== delete file
     let clean_c = brown::remove_file("tests/kwh/file_name.ext");
     assert!(clean_c.is_ok());
     let clean_d = brown::remove_file("tests/kwh/file_name2.ext");
     assert!(clean_d.is_ok());
    
     let files_clean = brown::get_dirs(&parent_path).unwrap();
    assert_eq!(files_clean.len(),2); // still a and b are dirs

     //== noe del dirs
    let clean_a = brown::remove_dir("tests/kwh/a"); 
    assert!(clean_a.is_ok()); // deleted dir
    let clean_b = brown::remove_dir("tests/kwh/b"); 
    assert!(clean_b.is_ok()); // deleted dir
    
    let dirs_clean = brown::get_files(&parent_path);
    assert!(dirs_clean.is_err()); // error since len = 0
    
   
    //====delete parent
    let parent_del = brown::remove_dir(&parent_path);
    assert!(parent_del.is_ok());
}
