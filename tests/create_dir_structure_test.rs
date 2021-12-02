use brown as bro;

#[cfg(test)]
#[test]
fn use_fn(){
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
    
    let _ = brown::remove_dir_brute("./data");
    let _ = brown::remove_dir_brute("./data_with");
    let _ = brown::remove_dir_brute("./site/images");
    let _ = brown::remove_dir_brute("./hulkfolder/templates"); 
    let _ = brown::remove_dir_brute("./hulkfolder"); 
     
}