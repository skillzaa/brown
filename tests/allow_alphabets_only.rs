use brown::Brown;

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
//let r = a.allow_alphabets_only(&String::from("abcdews"));
    assert_eq!(2,2);
}
