use brown::Hdir;

#[cfg(test)]
#[test]
fn create_delete_files() {
    let hdir = Hdir::new("tests/trial".to_string());
    
    // create once
    let r = hdir.create_file("tests/bac.html");
    assert!(!r.is_err());
    
    // create twice
    let rr = hdir.create_file("tests/bac.html");
    assert!(rr.is_err()); // its error cant make same file twice
    // clean up
    let del = hdir.delete_file("tests/bac.html");
    assert!(!del.is_err());

}
