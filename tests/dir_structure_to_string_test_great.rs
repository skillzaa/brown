use brown as bro;
mod setup;
#[cfg(test)]
#[test]
fn use_fn(){
let _ = setup::build_up_dirs();
let mut output:Vec<String> = Vec::new();
//--level 0 here--the parent folder name
// --check if thr source folder exist
//-- if yes get it in there--later
//--now get level one-- this level takes string while all the itterations after this takes DirEntry thus this one is kept seperate for simpleicity.
let level_one_de = 
bro::get_dirs(setup::PARENTFOLDER).unwrap();

let mut level_one_str = 
bro::direntry_to_path_all(&level_one_de, true)
.unwrap();
// get level one string into output
output.append(&mut level_one_str);
assert_eq!(output,[
    "./tests/delme/b",
    "./tests/delme/a",
    "./tests/delme/c",]);

//==========level -2
let level_two_de = 
bro::get_dirs_all(&level_one_de).unwrap();
let mut level_two_str = 
bro::direntry_to_path_all(&level_two_de, true).unwrap();
output.append(&mut level_two_str);
assert_eq!(output,[
    "./tests/delme/b",
    "./tests/delme/a",
    "./tests/delme/c",
    "././tests/delme/a/a2",
    "././tests/delme/a/a1",
    "././tests/delme/a/a3",
    "././tests/delme/c/c1",
    "././tests/delme/c/c2",]);
//==========level -3
let level_three_de = 
bro::get_dirs_all(&level_two_de).unwrap();
let mut level_three_str = 
bro::direntry_to_path_all(&level_three_de, true).unwrap();
output.append(&mut level_three_str);

assert_eq!(output,[
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
 let _ = setup::tear_down();
// println!("{:#?}",output);
}