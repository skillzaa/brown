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
fn a() {
let a = Abc::new();
    let foo = a.create_file("./tests/foo.txt");
    let a  = a.path_exists("./tests/foo.txt");
    assert_eq!(true,a);
    
//let r = a.allow_alphabets_only(&String::from("abcdews"));
}
#[test]
fn b() {
let a = Abc::new();
    a.create_file("./tests/foo.txt");
    let b  = a.path_exists("./tests/foo.txt");
    assert_eq!(true,b);
    a.delete_file("./tests/foo.txt");

    let d  = a.path_exists("./tests/foo.txt");
    assert_eq!(false,d);
}
#[test]
fn c() {
let a = Abc::new();
    let del_result = a.delete_file("./not_exist.txt");
    assert!(del_result.is_err());
}
