use brown::navbar::NavBar;

#[test]
pub fn something(){
    let navbar = 
    NavBar::new("delme/a/a1/a11/a111");
    assert!(navbar.is_ok());
    let navbar = navbar.unwrap();
    println!("{:#?}",navbar.gen_navbar());
}