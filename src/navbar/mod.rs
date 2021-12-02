use std::io::{Error};

mod urltolinks;
use urltolinks::UrlToLinks;
pub struct NavBar {
    urltolinks:UrlToLinks,
}
impl NavBar{
    pub fn new(url:&'static str)->Result<Self,Error>{
        let urltolinks = UrlToLinks
        ::new(url.to_string());
        match urltolinks {
            Ok(urltolinks)=>{
                Ok(NavBar{urltolinks})
            },
            Err(e)=> Err(e),
        }
    }
    pub fn gen_navbar(&self)->String{
        let mut navbar = String::new();
        let links_raw = self.urltolinks
        .get_links();
        let links = 
        links_raw.into_iter().collect::<String>();
        navbar.push_str(navbar_start());
        
        navbar.push_str(&links);
        navbar.push_str(navbar_end());
        navbar
    }
}

fn navbar_start()->&'static str{
let r = r#"<header id='header'><nav class='links' style='--items: 1;'>"#;
r    
}
fn navbar_end()->&'static str{
 let r = r#"</nav></header>"#;
 r
}

#[cfg(test)]
mod tests {
 use super::*;
#[test]
fn test_nav_final(){
    let url = "delme/a/a1/a11/a111";
let navbar = 
NavBar::new(url).unwrap();
let output = r#"<header id='header'><nav class='links' style='--items: 1;'><a href='./delme/index.html'>delme</a><a href='./delme/a/index.html'>a</a><a href='./delme/a/a1/index.html'>a1</a><a href='./delme/a/a1/a11/index.html'>a11</a><a href='./delme/a/a1/a11/a111/index.html'>a111</a></nav></header>"#;
assert_eq!(output,navbar.gen_navbar());
}
}

