use brown;

#[cfg(test)]
#[test]
fn a() {
    brown::create_file("./tests/foo.txt");
    let a  = brown::path_exists("./tests/foo.txt");
    assert_eq!(true,a);
    
//let r = a.allow_alphabets_only(&String::from("abcdews"));
}
#[test]
fn b() {
    brown::create_file("./tests/foo.txt");
    let b  = brown::path_exists("./tests/foo.txt");
    assert_eq!(true,b);
    brown::delete_file("./tests/foo.txt");

    let d  = brown::path_exists("./tests/foo.txt");
    assert_eq!(false,d);
}
#[test]
fn c() {
    let del_result = brown::delete_file("./not_exist.txt");
    assert!(del_result.is_err());
}
