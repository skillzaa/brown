// use brown::qndr;
// use brown::tasks;
// #[test]
// fn run(){
//     let url = "delme/a/a1/a11/a111";
//     let exploded = 
//     qndr::explode(url,'/');
//     println!("{:#?}",exploded);
//     let mut output = String::new();
// ////////////////////////////////////////
// for i in 0..exploded.len(){
// let x = get_link(&exploded,i);
// output.push_str(&x);
// } 
// println!("{:#?}",output);
// }


// #[test]
// fn test_get_segments(){
//     let url = "delme/a/a1/a11/a111";
//     let exploded = 
//     qndr::explode(url,'/');
//     let counter = 99;
//     let x =get_segments(&exploded, counter);
//     println!("x======={:?}",x);
// }
// fn get_segments(exploded:&Vec<String>,counter:usize)->String{
// let mut local_counter = 0;
// let mut buffer = String::new();
// for i in exploded {
//     buffer.push_str(i);
//     buffer.push_str("/");
//     local_counter = local_counter +1;
//         if local_counter >= counter {
//             return buffer;
//         }
// }
// buffer
// }
// #[test]
// fn get_nth_segment_test(){
//     let mut url:Vec<String> = Vec::new();
//     url.push("delme".to_string());
//     url.push("a".to_string());
//     url.push("a1".to_string());
//     url.push("a11".to_string());
//     url.push("a111".to_string());

//     assert_eq!(&url[0],"delme");
//     assert_eq!(&url[1],"a");
//     assert_eq!(&url[2],"a1");
//     assert_eq!(&url[3],"a11");
//     assert_eq!(&url[4],"a111");

// }

// fn final_product()->&'static str{
// let r =r#"
// <a href='./delme/index.html'>delme</a>    
// <a href='./delme/a/index.html'>a</a>    
// <a href='./delme/a/a1/index.html'>a1</a>    
// <a href='./delme/a/a11/index.html'>a11</a>    
// <a class='active' href='./delme/a/a111/index.html'>a111</a> 
// <a href='./delme/a/a111/kid1/index.html'>kid1</a> 
// <a href='./delme/a/a111/kid2/index.html'>kid2</a>
// "#;
// r
// }

#[test]
fn open(){
    let a = "delme/a/b/c/d".to_string();
    let split = a.split('/');
    for s in split {
        println!("{:#?}",s);

    }

}