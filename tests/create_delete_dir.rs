use brown::Hdir;

#[cfg(test)]
#[test]
fn create_delete_test() {
    let hdir = Hdir::new("tests/trial".to_string());
    // create once
    let r = hdir.create();
    assert!(!r.is_err());
    // delete once
    let foo = hdir.delete();
    assert!(!foo.is_err()); // not error
    // delete twice
    let again = hdir.delete();
    assert!(again.is_err()); // is error
}
