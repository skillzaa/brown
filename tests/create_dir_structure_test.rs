use brown as bro;

#[cfg(test)]
#[test]
fn use_fn(){
    use bro::remove_dir_brute;
    let paths_list = vec!
            [ 
                "/hulkfolder" ,
                "./data2", 
                "si?te" ,
                "data", 
                "data_with", 
                "site/images", 
                "hulkfolder/templates" ,
            ];
let a = bro::create_dir_structure
(&paths_list);
assert!(a.is_ok());
    //====== tests
    assert_eq!(true,brown::path_exists("./data"));
    assert_eq!(true,brown::path_exists("./data_with"));
    assert_eq!(true,brown::path_exists("./site/images"));
    assert_eq!(true,brown::path_exists("./hulkfolder/templates")); 
// cleanup    
    // i can also just remove the base folders
    let data_removed = 
    brown::remove_dir_brute("data");
        assert!(data_removed.is_ok());
    let _ = brown::remove_dir_brute("data_with");
    let _ = brown::remove_dir_brute("site/images");
    let _ = brown::remove_dir_brute("./hulkfolder/templates"); 
    let _ = brown::remove_dir_brute("./hulkfolder"); 
    let site_removed = 
    remove_dir_brute("site");
    assert!(site_removed.is_ok()); 
}