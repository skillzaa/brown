use brown;

#[cfg(test)]
#[test]
fn  get_files_by_ext_test() {
    let parent_path = "tests/cuw";
    // create parent folder
    let parent = brown::create_dir(parent_path);
    assert!(parent.is_ok());
    
    //-- add 4 file a n b have .md c is .html
    // Have 2 md files and 1 hmtl
    let a = brown::create_file("tests/cuw/file_nameA.md");
    assert!(a.is_ok());
    let b = brown::create_file("tests/cuw/file_nameB.md");
    assert!(b.is_ok());
    let c = brown::create_file("tests/cuw/file_nameC.html");
    assert!(c.is_ok());
    
    //-- this is get FILEs
    let ulti = brown::get_files(&parent_path).unwrap();
    assert_eq!(ulti.len(),3); // just c and d are files
    //-- this is get FILEs
    let g_f_b_e = brown::get_files_by_ext(&parent_path,"md").unwrap();
    assert_eq!(g_f_b_e.len(),2); // just c and d are files
    //-- this is get FILEs
    let g_f_b_e = brown::get_files_by_ext(&parent_path,"html").unwrap();
    assert_eq!(g_f_b_e.len(),1); // just c and d are files
    // println!("{:?}",ulti);

    //---now clean up=======del all folders
    
    //==== delete file
    let clean_a = brown::remove_file("tests/cuw/file_nameA.md");
    assert!(clean_a.is_ok());
    let clean_b = brown::remove_file("tests/cuw/file_nameB.md");
    assert!(clean_b.is_ok());
    //----- both md files are deleted check them now
    let md_del = brown::get_files_by_ext(parent_path,"md");
    assert!(md_del.is_err()); //since found no md
    let find_html = brown::get_files_by_ext(parent_path,"html");
    assert!(find_html.is_ok());
    let unwrapped_find_html = find_html.unwrap(); 
    assert_eq!(unwrapped_find_html.len(),1); //since found no md
    //----------------------------------------------
    let clean_c = brown::remove_file("tests/cuw/file_nameC.html");
    assert!(clean_c.is_ok());
   
    let no_html = brown::get_files_by_ext(&parent_path,"html");
    assert!(no_html.is_err()); // since the vec has 0 entries    

    //====delete parent
    let parent_del = brown::remove_dir(&parent_path);
    assert!(parent_del.is_ok());
}
