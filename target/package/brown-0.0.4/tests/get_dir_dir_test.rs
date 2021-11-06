use brown;

#[cfg(test)]
#[test]
fn a() {
brown::create_dir("tests/abc").unwrap();
let v = brown::get_dir_from_dir("tests/abc");
assert_eq!(v.len(),0);
let create_one = brown::create_dir("tests/abc/xyz").unwrap();
let v = brown::get_dir_from_dir("tests/abc");
assert_eq!(v.len(),1);
let create_one = brown::create_dir("tests/abc/abc").unwrap();
let v = brown::get_dir_from_dir("tests/abc");
assert_eq!(v.len(),2);
let create_one = brown::create_dir("tests/abc/efg").unwrap();
let v = brown::get_dir_from_dir("tests/abc");
assert_eq!(v.len(),3);
//=================================
  let create_one = brown::remove_dir("tests/abc/abc");
  let create_one = brown::remove_dir("tests/abc/efg");
  let create_one = brown::remove_dir("tests/abc/xyz");
  
  let vv = brown::get_dir_from_dir("tests/abc");
  assert_eq!(vv.len(),0);
  //finally also remove this
  let finally = brown::remove_dir("tests/abc").unwrap();
//   println!("==> {:?}",v);
    assert_eq!((),finally);   
}
