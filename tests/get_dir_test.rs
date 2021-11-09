use brown as bro;
mod buildup;

#[cfg(test)]
#[test]
fn get_dir_test() {
    //remove old remanants 
    let _ = buildup::tear_down();
    // assert!(del_old.is_ok());
    let bup  = buildup::build_up();
    assert!(bup.is_ok()); // if this is ok all 3 files and folders have been generated so no need to check them again
    
     let dirs_res = bro::get_dirs("delme");
    assert!(dirs_res.is_ok());
    let dirs = dirs_res.unwrap();
    //======first test here
    assert_eq!(dirs.len(),3); // just a and b are dirs
    
    let rem_dirs_opt = buildup::remove_dirs();
    // match rem_dirs_opt {
    //     Err(e)=>{
    //         println!("{:?}",e);
    //     },
    //     _=>{},
    // }
    assert!(rem_dirs_opt.is_err()); //since have  no dirs so it returns error ifit find no dirs.

}
