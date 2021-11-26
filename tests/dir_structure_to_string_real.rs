use brown::tasks;
mod setup;
#[cfg(test)]
#[test]
fn loop_test(){
let _ = setup::build_up_dirs();
let output = tasks::dir_structure_to_string
::run(setup::PARENTFOLDER).unwrap();

let _ = setup::tear_down();
// println!("{:#?}",output);
check_final(&output);
}

fn check_final(output:&Vec<String>)->bool {
let v: Vec<&str> = output.iter().map(|s| &**s).collect();
  
assert_eq!(v,[
        "tests/delme/",
        "./tests/delme/b",
        "./tests/delme/a",
        "./tests/delme/c",
        "././tests/delme/a/a2",
        "././tests/delme/a/a1",
        "././tests/delme/a/a3",
        "././tests/delme/c/c1",
        "././tests/delme/c/c2",
        "./././tests/delme/c/c2/c22",
        "./././tests/delme/c/c2/c21",
    ]);
    true
}