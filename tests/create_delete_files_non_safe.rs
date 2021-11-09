use brown;

#[cfg(test)]
#[test]
fn create_remove_files_non_safe() {
    
    // create once
    // first time safe and not safe are same
    let r = brown::create_file_brute("tests/bac.html");
    assert!(r.is_ok());
    
    // create twice -- it should create the same file again
    let rr = brown::create_file_brute("tests/bac.html");
    assert!(rr.is_ok()); // its OK can make same file twice
    // clean up
    let del = brown::remove_file("tests/bac.html");
    assert!(del.is_ok());

}
