use brown::Hdir;

#[cfg(test)]
#[test]
fn get_entries_test() {
   
   let hdir = Hdir::new("./src".to_string());
   let entries = hdir.get_dir();
   match entries {
       Ok(ent)=>{
           println!("{:?}",ent);
       },
       Err(e)=>println!("{:?}",e),
   }
    
}
//#[should_panic]
