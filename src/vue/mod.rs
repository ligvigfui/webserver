use std::net::TcpStream;

use crate::*;

pub fn routing(stream: &mut TcpStream, request: Request){
    use Method as M;
    match (&request.method, request.path) {
        (M::GET, "" | "/index.html") => handle_page_return(stream, "200 OK", Some(vec!["Content-type: text/html"]), &("vue/dist/index.html")),
        (M::GET, path) => match path.split(".").last() {
            Some("png") | Some("jpg") | Some("jpeg") | Some("gif") | Some("svg") => {
                handle_image(stream, path);
            },
            Some("css") => {
                handle_page_return(stream, "200 OK", Some(vec!["Content-type: text/css"]), &format!("vue/{}", request.path))
            },
            Some("js") => {
                handle_page_return(stream, "200 OK", Some(vec!["Content-type: text/javascript; charset=UTF-8"]), &format!("vue/{}", request.path))
            },
            Some("html") => {
                handle_page_return(stream, "200 OK", Some(vec!["Content-type: text/html"]), &format!("vue/{}", request.path))
            },
            None | Some(_) => {
                handle_page_return(stream, "200 OK", None, &format!("vue/{}", request.path))
            }
        },
        _ => {
            println!("404 Page not found");
            handle_page_return(stream, "404 NOT FOUND", None, &("en/404.html"));},
    }
}

fn handle_image(stream: &mut TcpStream, path: &str) {
    if DEBUG >= DebugLevel::HIGH {
        println!("Image request: pages/vue{}", path);
    }
    match handle_image_inner(stream, format!("pages/vue{}", path)) {
        Err(e) => {
            println!("{}", e);
        }
        Ok(_) => {}
    }
}