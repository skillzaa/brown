// use crate::qndr;
use std::io::{ErrorKind,Error};
// in a lib you talk to siblings through mod.rs 
pub struct UrlToLinks{
  url:&'static str,
  segments:Vec<String>
}
impl UrlToLinks {
    pub fn new(url:&'static str)->Result<Self,Error>{
        
        let segments_str:Vec<&str> = 
        url.split('/').collect();
        
         if segments_str.len() < 1 {
             let e = Error::new(ErrorKind::InvalidData,"url seems faulty");
             return Err(e);
         }else {
            let segments = 
            segments_str.iter()
            .map(|s|s.to_string()).collect(); 
            Ok(UrlToLinks{url,segments})
         }
    }
    pub fn get_segments(&self,counter:usize)->String{
    // if counter < 0 || counter >     
    let mut local_counter = 0;
    let mut buffer = String::new();
        for i in &self.segments {
            buffer.push_str(&i);
            buffer.push_str("/");
            if local_counter >= counter {
                return buffer;
            }
            local_counter = local_counter +1;
        }
        buffer
        }
    pub fn get_link(&self,counter:usize)->String{ 
        let segments = self.get_segments(counter);
        let item = self.get_item(counter);
                
        let link = 
        format!("<a href='./{}'>{}</a>",segments,item);      
        link
        }
    pub fn get_links(&self)->Vec<String>{
        let mut links:Vec<String> = Vec::new();
        let mut counter = 0;
        for l in &self.segments {
            let link = self.get_link(counter);
            counter = counter + 1;
            links.push(link);
        }
        links                
    }
    pub fn get_item(&self,counter:usize)
    ->String{
    let mut buffer = String::new();
    let seg = self.segments.clone(); 
    seg[counter].clone()
    
    }
}


#[cfg(test)]
mod tests {
use super::*;
#[test]
fn test_get_segments(){
let url = "delme/a/a1/a11/a111";
let r = UrlToLinks::new(url);
    match r {
        Ok(urltolinks)=>{
            assert_eq!(urltolinks.get_segments(0),"delme/");
            assert_eq!(urltolinks.get_segments(1),"delme/a/");
            assert_eq!(urltolinks.get_segments(2),"delme/a/a1/");
            assert_eq!(urltolinks.get_segments(3),"delme/a/a1/a11/");
            assert_eq!(urltolinks.get_segments(4),"delme/a/a1/a11/a111/");
            
        },
        Err(_e)=>{}
    }
}
#[test]
fn test_get_items(){
let url = "delme/a/a1/a11/a111";
let r = UrlToLinks::new(url);
    match r {
        Ok(urltolinks)=>{
            assert_eq!(urltolinks.get_item(0),"delme");
            assert_eq!(urltolinks.get_item(1),"a");
            assert_eq!(urltolinks.get_item(2),"a1");
            assert_eq!(urltolinks.get_item(3),"a11");
            assert_eq!(urltolinks.get_item(4),"a111");
            
        },
        Err(_e)=>{}
    }
}

}