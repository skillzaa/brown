use brown::Hdir;

#[cfg(test)]
#[test]
fn create_delete_test() {
    let hdir = Hdir::new().unwrap();
    let path = "./tests/trial";
    // create once
    let r = hdir.create_dir(path);
    assert!(!r.is_err());
    // delete once
    let foo = hdir.remove_dir(path);
    assert!(!foo.is_err()); // not error
    // delete twice
    let again = hdir.remove_dir(path);
    assert!(again.is_err()); // is error
}
