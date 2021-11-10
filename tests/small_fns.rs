use brown as bro;
mod buildup;

#[cfg(test)]
#[test]
fn get_dir_test() {
    //remove old remanants 
    let _ = buildup::tear_down();
    let bup  = buildup::build_up();
    assert!(bup.is_ok()); 
    let files_res = bro::get_files("./delme");
    assert!(files_res.is_ok());
    let files = files_res.unwrap();
        for file in files {
            let ext_res = bro::get_ext(&file);
            assert!(ext_res.is_ok());
            let ext = ext_res.unwrap();
            assert_eq!(ext,"md");

            let is_file = bro::is_file(&file).unwrap();
            assert_eq!(is_file,true);

            let file_name = bro::get_file_name(&file);
            // println!("{:?}",file_name.unwrap());
        }

buildup::tear_down();        
}
