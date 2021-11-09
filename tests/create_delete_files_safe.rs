use brown;

#[cfg(test)]
#[test]
fn create_remove_files_safe() {
    
    // create once
    let r = brown::create_file("tests/bac.html");
    assert!(r.is_ok());
    
    // create twice
    let rr = brown::create_file("tests/bac.html");
    assert!(rr.is_err()); // its error cant make same file twice
    // clean up
    let del = brown::remove_file("tests/bac.html");
    assert!(del.is_ok());

}
