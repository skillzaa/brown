use brown;

#[cfg(test)]
#[test]
fn create_dir_test() {
    brown::create_dir("tests/123dew").unwrap();
    let exists  = brown::path_exists("./tests/123dew");
    assert_eq!(true,exists);
    //clean up
    let foo = brown::remove_dir("tests/123dew").unwrap();
    assert_eq!((),foo); // This took me 15 mintues

    
}
#[test]
#[should_panic]
fn remove_dir_test() {
    brown::create_dir("./tests/xyz");
    let b  = brown::path_exists("./tests/xyz");
    assert_eq!(true,b);
    brown::remove_dir("./tests/abc");

    let d  = brown::path_exists("./tests/xyz");
    assert_eq!(false,d);
}
#[test]
fn dir_not_present() {
    let del_result = brown::remove_dir("./notexist");
    assert!(del_result.is_err());
    
}
