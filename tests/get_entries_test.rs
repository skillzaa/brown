use brown::Hdir;

#[cfg(test)]
#[test]
fn get_entries_test() {
    let hdir = Hdir::new().unwrap();
    let parent_path = "tests/xxx";
    // create parent folder
    let parent = hdir.create_dir(parent_path);
    assert!(!parent.is_err());
    
    // now create 3 dir ie a  , b , c
    let a = hdir.create_dir("tests/xxx/a");
    assert!(!a.is_err());
    let b = hdir.create_dir("tests/xxx/b");
    assert!(!a.is_err());
    let c = hdir.create_dir("tests/xxx/c");
    assert!(!a.is_err());
    ///////////-- add a file
    let d = hdir.create_file("tests/xxx/file_name.ext");
    assert!(!d.is_err());
    let ulti = hdir.get_entries(&parent_path).unwrap();
    assert_eq!(ulti.len(),4);
    println!("{:?}",ulti);

    //---now clean up=======del all folders
    let clean_a = hdir.remove_dir("tests/xxx/a"); 
    assert!(clean_a.is_ok()); // deleted dir
    let clean_b = hdir.remove_dir("tests/xxx/b"); 
    assert!(clean_b.is_ok()); // deleted dir
    let clean_c = hdir.remove_dir("tests/xxx/c");
    assert!(clean_c.is_ok()); // deleted dir
    
    let dirs_clean = hdir.get_entries(&parent_path).unwrap();
    assert_eq!(dirs_clean.len(),1); // just the file left
    
    //==== delete file
    let clean_f = hdir.remove_file("tests/xxx/file_name.ext");
    assert!(clean_f.is_ok());

    let file_clean = hdir.get_entries(&parent_path);
    assert!(file_clean.is_err()); // since the vec has 0 entries    

    //====delete parent
    let parent_del = hdir.remove_dir(&parent_path);
    assert!(parent_del.is_ok());
}
