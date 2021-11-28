use crate::qndr;
use std::io::{ErrorKind,Error};

mod urltolinks;
use urltolinks::UrlToLinks;
pub struct NavBar {
    urltolinks:UrlToLinks,
}
impl NavBar{
    pub fn new(url:&'static str)->Result<Self,Error>{
        let urltolinks = UrlToLinks
        ::new(url);
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
        //navbar.push_str(navbar_start());
        
        navbar.push_str(&links);
        //navbar.push_str(navbar_end());
        navbar
    }
}

fn navbar_start()->&'static str{
let r = r#"<header id="header">
	<nav class="links" style="--items: 1;">
    "#;
r    
}
fn navbar_end()->&'static str{
 let r = r#"</nav></header>"#;
 r
}


