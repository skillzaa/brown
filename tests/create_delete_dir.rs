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
    let foo = a.create_dir("./tests/abc").unwrap();
    let a  = a.path_exists("./tests/abc");
    assert_eq!(true,a);
    
//let r = a.allow_alphabets_only(&String::from("abcdews"));
}
#[test]
fn b() {
let a = Abc::new();
    a.create_dir("./tests/xyz");
    let b  = a.path_exists("./tests/xyz");
    assert_eq!(true,b);
    a.delete_dir("./tests/abc");

    let d  = a.path_exists("./tests/abc");
    assert_eq!(false,d);
}
// #[test]
// fn c() {
// let a = Abc::new();
//     let del_result = a.delete_file("./not_exist.txt");
//     assert!(del_result.is_err());
// }
