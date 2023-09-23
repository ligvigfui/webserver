use std::net::TcpStream;

use crate::*;
use server_functions as SF;

pub fn routing(stream: &mut TcpStream, request: Request){
    use Method as M;
    match (&request.method, request.path) {
        (M::GET, "") => handle_page_return(stream, "200 OK", Some(vec![]), "vue/dist/index.html"),
        (M::GET, path) => match path.split(".").last() {
            Some("png") | Some("jpg") | Some("jpeg") | Some("gif") | Some("svg") => {
                handle_image(stream, path);},
            Some("css") | Some("html") => {
                handle_page_return(stream, "200 OK", Some(vec![]), &format!("vue/{}", request.path))},
            Some("js") => {
                SF::handling::handle_page_return(stream, 
                "200 OK", Some(vec!["Content-type: text/javascript; charset=UTF-8"]), &format!("vue/{}", request.path))},
            None | Some(_) => {
                handle_page_return(stream, "200 OK", None, &format!("vue/{}", request.path))}
        },
        _ => {
            println!("404 Page not found");
            handle_page_return(stream, "404 NOT FOUND", None, &("en/404.html"));
        },
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

fn handle_page_return(stream: &mut TcpStream, status: &str, headers: Option<Vec<&str>>, path: &str) {
    let extension = path.split(".").last().unwrap();
    let mut headers2 = headers.unwrap();
    let string = format!("Content-type: text/{}", extension);
    headers2.push(&string);
    SF::handling::handle_page_return(stream, status, Some(headers2), path);
}