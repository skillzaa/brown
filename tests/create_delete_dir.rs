use brown::Brown;
use std::io::{Error, ErrorKind};

#[derive(Debug)]

struct Abc { }

impl Abc {
    fn new()->Abc{
        Abc {}
    }
}
impl Brown for Abc {}

#[cfg(test)]
#[test]
fn create_dir_test() {
let a = Abc::new();
    let created = a.create_dir("tests/123dew").unwrap();
    let exists  = a.path_exists("./tests/123dew");
    assert_eq!(true,exists);
    //clean up
    let foo = a.remove_dir("tests/123dew").unwrap();
    assert_eq!((),foo); // This took me 15 mintues

    
}
#[test]
#[should_panic]
fn remove_dir_test() {
let a = Abc::new();
    a.create_dir("./tests/xyz");
    let b  = a.path_exists("./tests/xyz");
    assert_eq!(true,b);
    a.remove_dir("./tests/abc");

    let d  = a.path_exists("./tests/xyz");
    assert_eq!(false,d);
}
#[test]
fn dir_not_present() {
let a = Abc::new();
    let del_result = a.remove_dir("./notexist");
    assert!(del_result.is_err());
    
}
