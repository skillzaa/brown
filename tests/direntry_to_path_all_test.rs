use brown as bro;
mod setup;
#[test]
fn uno() {
   let _ = setup::build_up_dirs();
   let dirs = 
   bro::get_dirs(setup::PARENTFOLDER).unwrap();
   let paths = 
   bro::direntry_to_path_all(&dirs, true)
   .unwrap(); 
   let test_data = [
      "./tests/delme/a",
      "./tests/delme/b",
      "./tests/delme/c"
      ];
      for p in paths {
         let s = &p.as_str();
         assert!(test_data.contains(s));
      }
   let _ = setup::tear_down();
}
