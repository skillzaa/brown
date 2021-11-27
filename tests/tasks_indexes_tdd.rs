use brown as bro;
use brown::tasks;
use brown::util;
use brown::qndr;
mod setup;
#[cfg(test)]
#[test]
fn indexes_test(){
let _ = setup();
//==== Step-1 Get Vec of target folder paths
let st = tasks::dir_structure_to_string
::run("delme").unwrap();
//  println!("st{:#?}",st);
//  println!("step:1{:#?}",data_dir_structure_to_string());
assert_eq!(st,data_dir_structure_to_string());
//=== Step-2 remove the ./
let dot_slash_removed = 
util::paths_remove_dot_slash(&st).unwrap();
assert_eq!(dot_slash_removed,data_dot_slash_removed());
// println!("dot_slash_removed{:#?}",dot_slash_removed);

let indexes = indexes_controller(&dot_slash_removed); 

//let _ = tear_down();
//=============== TEST ENDS
}
//===========================================
//===========================================
//===========================================
//===========================================
fn data_dir_structure_to_string()->Vec<String>{
    let mut v:Vec<String> = Vec::new();
    v.push(String::from("delme"));
    v.push(String::from("./delme/b"));
    v.push(String::from("./delme/a"));
    v.push(String::from("./delme/c"));
    v.push(String::from("././delme/a/a2"));
    v.push(String::from("././delme/a/a1"));
    v.push(String::from("././delme/a/a3"));
    v.push(String::from("././delme/c/c1"));
    v.push(String::from("././delme/c/c2"));
    v.push(String::from("./././delme/c/c2/c22"));
    v.push(String::from("./././delme/c/c2/c21"));
    v
}
fn tear_down(){
    bro::remove_dir_brute("delme");
}
fn setup()->bool{
    let PARENTFOLDER = "delme";
    // delete old
    let _= brown::remove_dir_brute(PARENTFOLDER);
    // parent folder
    let _aa = brown::create_dir(PARENTFOLDER).unwrap();
  
  // now 3 folders-- 
  brown::create_dir(format!("{}{}",PARENTFOLDER,"/a").as_str()).unwrap();
  brown::create_dir(format!("{}{}",PARENTFOLDER,"/a/a1").as_str()).unwrap();
  brown::create_dir(format!("{}{}",PARENTFOLDER,"/a/a2").as_str()).unwrap();
  brown::create_dir(format!("{}{}",PARENTFOLDER,"/a/a3").as_str()).unwrap();
  brown::create_dir(format!("{}{}",PARENTFOLDER,"/b").as_str()).unwrap();
  brown::create_dir(format!("{}{}",PARENTFOLDER,"/c").as_str()).unwrap();
  brown::create_dir(format!("{}{}",PARENTFOLDER,"/c/c1").as_str()).unwrap();
  brown::create_dir(format!("{}{}",PARENTFOLDER,"/c/c2").as_str()).unwrap();
  brown::create_dir(format!("{}{}",PARENTFOLDER,"/c/c2/c21").as_str()).unwrap();
  brown::create_dir(format!("{}{}",PARENTFOLDER,"/c/c2/c22").as_str()).unwrap();
  
  true
  
}
fn data_dot_slash_removed()->Vec<String>{
let mut v:Vec<String> = Vec::new();
    v.push(String::from("delme"));
    v.push(String::from("delme/b"));
    v.push(String::from("delme/a"));
    v.push(String::from("delme/c"));
    v.push(String::from("delme/a/a2"));
    v.push(String::from("delme/a/a1"));
    v.push(String::from("delme/a/a3"));
    v.push(String::from("delme/c/c1"));
    v.push(String::from("delme/c/c2"));
    v.push(String::from("delme/c/c2/c22"));
    v.push(String::from("delme/c/c2/c21"));
    v
}
// fn get_default_nav()->&'static str{
//     let a = r#"
//     <header id="header">
// 	<nav class="links" style="--items: 1;">
// 		<a href="./index.html">Home</a>
// 		<span class="line"></span>
// 	</nav>
// </header>
//     "#;
//     a
// }

fn indexes_controller(dot_slash_removed:&Vec<String>){
let mut final_nav:Vec<String> = Vec::new();    
    for nav_bar_str in dot_slash_removed{
        let mut output = String::new();    
        let exploded = 
        qndr::explode(nav_bar_str, '/');
            if exploded.len() > 1 {
                for e in &exploded {
                    let nav_item 
                    = get_nav_item(&e,&exploded);  
                    output.push_str(&nav_item);   
                }
            
            }
        //=======================
        
    println!("output{:#?}",output);
    }
    // todo!();
}
#[test]
fn test_get_nav_bar(){
    let output = get_nav_bar("delme/a/a1"); 
    println!("output{:#?}",output);

}
fn get_nav_bar(url:&str)->String{
let mut output = String::new();    
    let exploded = 
        qndr::explode(url, '/');

        if exploded.len() > 1 {
                for e in &exploded {
                    let nav_item 
                    = get_nav_item(&e,&exploded);  
                    output.push_str(&nav_item);   
                }
            
            }
// println!("output{:#?}",output);
output
}

fn get_nav_item(item:&String,exploded:&Vec<String>)->String{
let mut output = String::from("<a href='./");
for i in exploded {
    if i == item {
        output.push_str(&item);
        output.push_str("/index.html'>");
        output.push_str(&item);
        output.push_str("</a>");

    }else {

        output.push_str(&item);
        output.push_str("/");
               // format!("<a href='./{}/index.html'>{}</a>",item,item);
        // output.push_str("<br/>");
        return output;
    }
}
return output
}
