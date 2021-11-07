use brown::Hdir;

#[cfg(test)]
#[test]
fn create_remove_files() {
    let hdir = Hdir::new().unwrap();
    
    // create once
    let r = hdir.create_file("tests/bac.html");
    assert!(!r.is_err());
    
    // create twice
    let rr = hdir.create_file("tests/bac.html");
    assert!(rr.is_err()); // its error cant make same file twice
    // clean up
    let del = hdir.remove_file("tests/bac.html");
    assert!(!del.is_err());

}
