use std::net::TcpStream;

use crate::*;

pub fn routing(stream: &mut TcpStream, request: Request){
    use Method as M;
    match (&request.method, request.path) {
        (M::GET, "" | "/index.html") => handle_page_return(stream, "200 OK", None, &("vue/index.html")),
        (M::GET, path) => match path.split(".").last() {
            Some("png") | Some("jpg") | Some("jpeg") | Some("gif") | Some("svg") => {
                handle_image(stream, path);
            } 
            None | Some(_)=> {
                handle_page_return(stream, "200 OK", None, &format!("vue/{}", request.path))
            }
        },
        _ => {
            println!("404 Page not found");
            handle_page_return(stream, "404 NOT FOUND", None, &("en/404.html"));},
    }
}

fn handle_image(stream: &mut TcpStream, path: &str) {
    match handle_image_inner(stream, format!("pages/vue/assets{}", path)) {
        Err(e) => {
            println!("{}", e);
        }
        Ok(_) => {}
    }
}