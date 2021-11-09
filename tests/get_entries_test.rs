use brown;

#[cfg(test)]
#[test]
fn get_entries_test() {
    let parent_path = "tests/xxx";
    // create parent folder
    let parent = brown::create_dir(parent_path);
    assert!(!parent.is_err());
    
    // now create 3 dir ie a  , b , c
    let a = brown::create_dir("tests/xxx/a");
    assert!(!a.is_err());
    let b = brown::create_dir("tests/xxx/b");
    assert!(!b.is_err());
    let c = brown::create_dir("tests/xxx/c");
    assert!(!c.is_err());
    ///////////-- add a file
    let d = brown::create_file("tests/xxx/file_name.ext");
    assert!(!d.is_err());
    let ulti = brown::get_entries(&parent_path).unwrap();
    assert_eq!(ulti.len(),4);
    println!("{:?}",ulti);

    //---now clean up=======del all folders
    let clean_a = brown::remove_dir("tests/xxx/a"); 
    assert!(clean_a.is_ok()); // deleted dir
    let clean_b = brown::remove_dir("tests/xxx/b"); 
    assert!(clean_b.is_ok()); // deleted dir
    let clean_c = brown::remove_dir("tests/xxx/c");
    assert!(clean_c.is_ok()); // deleted dir
    
    let dirs_clean = brown::get_entries(&parent_path).unwrap();
    assert_eq!(dirs_clean.len(),1); // just the file left
    
    //==== delete file
    let clean_f = brown::remove_file("tests/xxx/file_name.ext");
    assert!(clean_f.is_ok());

    let file_clean = brown::get_entries(&parent_path);
    assert!(file_clean.is_err()); // since the vec has 0 entries    

    //====delete parent
    let parent_del = brown::remove_dir(&parent_path);
    assert!(parent_del.is_ok());
}
