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
let parent_folder = "cds";
    let a = brown::create_dir_structure::
    run(parent_folder,paths_list);
    //====== tests
    assert_eq!(true,brown::path_exists("cds/data"));
    assert_eq!(true,brown::path_exists("cds/site/images"));
    assert_eq!(true,brown::path_exists("cds/hulkfolder/templates"));
//===clean up
    let destroy = brown::remove_dir_brute(parent_folder).unwrap();
    assert_eq!(destroy,true);
}