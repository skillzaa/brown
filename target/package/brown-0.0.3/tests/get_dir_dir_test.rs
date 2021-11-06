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
let create_one = a.create_dir("tests/abc").unwrap();
let v = a.get_dir_from_dir("tests/abc");
assert_eq!(v.len(),0);
let create_one = a.create_dir("tests/abc/xyz").unwrap();
let v = a.get_dir_from_dir("tests/abc");
assert_eq!(v.len(),1);
let create_one = a.create_dir("tests/abc/abc").unwrap();
let v = a.get_dir_from_dir("tests/abc");
assert_eq!(v.len(),2);
let create_one = a.create_dir("tests/abc/efg").unwrap();
let v = a.get_dir_from_dir("tests/abc");
assert_eq!(v.len(),3);
//=================================
  let create_one = a.remove_dir("tests/abc/abc");
  let create_one = a.remove_dir("tests/abc/efg");
  let create_one = a.remove_dir("tests/abc/xyz");
  
  let vv = a.get_dir_from_dir("tests/abc");
  assert_eq!(vv.len(),0);
  //finally also remove this
  let finally = a.remove_dir("tests/abc").unwrap();
//   println!("==> {:?}",v);
    assert_eq!((),finally);   
}
