use brown;

#[cfg(test)]
#[test]
fn use_fn(){
    let paths_list = vec!
            [ 
                "/hulkfolder" ,
                "./data2", 
                "si?te" ,
                "data", 
                "site/images", 
                "hulkfolder/templates" ,
            ];
let _a = brown::create_dir_structure::
run(paths_list);
    //====== tests
    assert_eq!(true,brown::path_exists("./data"));
    assert_eq!(true,brown::path_exists("./site/images"));
    assert_eq!(true,brown::path_exists("./hulkfolder/templates"));

    
}