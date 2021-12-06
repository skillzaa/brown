use brown;

#[test]
fn create_delete_test() {
    let path = "tests/trial";
    // create once
    let r = brown::create_dir(path);
    assert!(r.is_ok());
    // create Twice
    // creating a dir is always safe there is no create_dir_brute fn. if we want thatwe have to 
    let r = brown::create_dir(path);
    assert!(r.is_err());
    // delete once
    let foo = brown::remove_dir(path);
    assert!(foo.is_ok()); // ok
    // delete twice
    let again = brown::remove_dir(path);
    assert!(again.is_err()); // is error
}
