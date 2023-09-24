use std::net::TcpStream;
use crate::{*, server_functions::deploy};


pub fn routing(stream: &mut TcpStream, request: Request, users: Arc<Vec<Mutex<User>>>){
    let request = match request.get_header("Accept-Language") {
        Some(x) => match x.contains("hu") {
            true => request.set_header("Accept-Language", "hu"),
            false => request.set_header("Accept-Language", "en"),
        },
        None => request.set_header("Accept-Language", "en"),
    };

    if DEBUG >= DebugLevel::LOW {
        println!("handeling - {}", request.path);
    }
    use Method as M;
    match (&request.method, request.path) {
        (_, path) if path.contains("/wedding") => 
            wedding::routing(stream, Request { path: path.split_once("/wedding").unwrap().1, .. request }),
        (_, path) if path.contains("/neptunCRF") => 
            neptunCRF::routing(stream, Request { path: path.split_once("/neptunCRF").unwrap().1, .. request }, users),
        (_, path) if path.contains("/vue") => 
            vue::routing(stream, Request { path: path.split_once("/vue").unwrap().1, .. request }),
        (_, "/favicon.ico") => handle_image(stream, "/favicon.ico"),
        (M::GET, "/") => handle_page_return(stream, "200 OK", None, "/index.html"),
        (M::GET, "/debug") => handle_debug(stream, request),
        (M::GET, "/deploy") => deploy(),
        _ => {
            println!("404 Page not found");
            handle_page_return(stream, "404 NOT FOUND", None,
            &(format!("{}/404.html", request.get_header("Accept-Language").unwrap())));},
    }
}