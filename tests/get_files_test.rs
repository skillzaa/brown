use brown;

#[cfg(test)]
#[test]
fn get_files_test() {
    let parent_path = "tests/kxt";
    // create parent folder
    let parent = brown::create_dir_safe(parent_path);
    assert!(parent.is_ok());
    
    // now create 2 dir ie a  , b 
    let a = brown::create_dir_safe("tests/kxt/a");
    assert!(a.is_ok());
    let b = brown::create_dir_safe("tests/kxt/b");
    assert!(b.is_ok());
    ///////////-- add a file
    let c = brown::create_file_safe("tests/kxt/file_name.ext");
    assert!(c.is_ok());
    let d = brown::create_file_safe("tests/kxt/file_name2.ext");
    assert!(d.is_ok());
    let ulti = brown::get_files(&parent_path).unwrap();
    assert_eq!(ulti.len(),2); // just c and d are files
    // println!("{:?}",ulti);

    //---now clean up=======del all folders
    let clean_a = brown::remove_dir("tests/kxt/a"); 
    assert!(clean_a.is_ok()); // deleted dir
    let clean_b = brown::remove_dir("tests/kxt/b"); 
    assert!(clean_b.is_ok()); // deleted dir
    
    let dirs_clean = brown::get_files(&parent_path).unwrap();
    assert_eq!(dirs_clean.len(),2); // still len = 2 since no file has been deleted
    
    //==== delete file
    let clean_c = brown::remove_file("tests/kxt/file_name.ext");
    assert!(clean_c.is_ok());
    let clean_d = brown::remove_file("tests/kxt/file_name2.ext");
    assert!(clean_d.is_ok());

    let file_clean = brown::get_files(&parent_path);
    assert!(file_clean.is_err()); // since the vec has 0 entries    

    //====delete parent
    let parent_del = brown::remove_dir(&parent_path);
    assert!(parent_del.is_ok());
}
