use brown;

#[cfg(test)]
#[test]
fn create_delete_test() {
    let path = "./tests/trial";
    // create once
    let r = brown::create_dir_safe(path);
    assert!(!r.is_err());
    // delete once
    let foo = brown::remove_dir(path);
    assert!(!foo.is_err()); // not error
    // delete twice
    let again = brown::remove_dir(path);
    assert!(again.is_err()); // is error
}
