
pub fn explode(input:&str,look_for:char)->Vec<String>{
let mut output:Vec<String> = Vec::new();
let mut c:String = String::new();
     
    for i in input.chars() {
        if i != look_for {
            c.push(i);
        }else {
            output.push(c);
            c = String::new();
        }
    }
    if c.len() > 1 {
        output.push(c);
        c = String::new();
    }
output
}

#[cfg(test)]
mod tests {
use super::*;
    #[test]
    fn explode_test(){
        let ex = 
        explode("home/sub1/sub2/sub3/sub4",
         '/');
         assert_eq!(ex,
            [   "home",
                "sub1",
                "sub2",
                "sub3",
            ]);
        //  println!("{:#?}",ex);

    }
    #[test]
    fn explode_test_02(){
        let ex = 
        explode("delme/a/a2",
         '/');
         assert_eq!(ex,
            [   "delme",
                "a",
                "a2",
            ]);
    }
    

}