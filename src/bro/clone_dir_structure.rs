use crate::BroPath;
use crate::bro;
use crate::BrownError;
use crate::path_exists;
/// The clone_dir_structure takes a source folder and destination folder names. It will then create a destination folder and clone the source folder/directory structure in to that. 
pub fn clone_dir_structure<'a>(source:&str,destination:&str)
->Result<Vec<String>,BrownError>{
//=== check if destination folder exists
match path_exists(source) {
   true=>{},
   false=>{return Err(BrownError::DirNotFound)},
}
//============= Step One    
let source_str = bro:: 
dir_structure_to_string(source)?;

//============ Step Two  
let bro_path = BroPath::new();
let paths_changed_parent = 
bro_path.change_destination(&source_str, source, destination)?;

let mutated = 
bro_path.remove_dot_slash_all(&paths_changed_parent)?;
//============ Step Three
let mutated_str:Vec<&str>  = 
mutated.iter().map(|s| &**s).collect();

let x = 
   bro::create_dir_structure
      (&mutated_str);

   match x {
   Ok(_item)=>{
            Ok(mutated) 
      },
   Err(e)=>{return Err(e);},
   }
}
    
mod tests {
use crate::test_helper::TestHelper;
use super::*;    
use super::super::*;
#[test]
fn manual_test(){
      let th = TestHelper::
      new("dir963");
      
      th.create_parent_dir();
      let _ = th.create_dir("a");
      let _ = th.create_dir("a/a1");
      let _ = th.create_dir("b");
      let _ = th.create_dir("b/b1");
      
let final_outcome  = 
clone_dir_structure("dir963","dir97")
.unwrap();
  println!("{:?}",final_outcome);    
      
      // println!("{:#?}",outcome);
      let compare = [
         "dir97",
         "dir97/b",
         "dir97/a",
         "dir97/b/b1",
         "dir97/a/a1",
      ];
      // in assert eq which ever you write first matter write the outcome first or it will raise errors about &str and String comaprison.
      assert_eq!(final_outcome,compare);
      th.tear_down();
}
}