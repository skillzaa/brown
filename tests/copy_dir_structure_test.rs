mod setup;
use brown as bro;

#[cfg(test)]
#[test]
fn loop_test(){
let _ = setup::build_up_dirs();
// let j = bro::
let output = bro::clone_dir_structure
(setup::PARENTFOLDER,"tests/abc/").unwrap();
// let _ = setup::tear_down();
println!("{:#?}",output);
//check_output(&output_one);
let _ = setup::tear_down();
let _ = brown::remove_dir_brute("tests/abc");
}

fn check_output(output:&Vec<&str>)->bool {
let v: Vec<&str> = output.iter().map(|s| &**s).collect();
  
assert_eq!(v,[
    "tests/abc/",
    "tests/abc/b",
    "tests/abc/a",
    "tests/abc/c",
    "tests/abc/a/a2",
    "tests/abc/a/a1",
    "tests/abc/a/a3",
    "tests/abc/c/c1",
    "tests/abc/c/c2",
    "tests/abc/c/c2/c22",
    "tests/abc/c/c2/c21",
]);
   
true
}