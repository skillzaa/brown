use brown as bro;
mod setup;

#[test]
fn get_dir_test() {
    //remove old remanants 
    let _ = setup::tear_down();
    // assert!(del_old.is_ok());
    let bup  = setup::build_up_dirs();
    assert!(bup.is_ok());
    
    let _ = setup::tear_down();        

}
