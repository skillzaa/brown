use brown;
use brown::tasks;

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
let a = tasks::create_dir_structure::
run(&paths_list);
assert!(a.is_ok());
    //====== tests
    assert_eq!(true,brown::path_exists("./data"));
    assert_eq!(true,brown::path_exists("./data_with"));
    assert_eq!(true,brown::path_exists("./site/images"));
    assert_eq!(true,brown::path_exists("./hulkfolder/templates"));   
}