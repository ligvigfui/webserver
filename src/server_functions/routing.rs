use std::net::TcpStream;
use crate::*;


pub fn routing(stream: &mut TcpStream, request: Request, users: Arc<Vec<Mutex<User>>>){
    let mut language = match request.get_header("Accept-Language") {
        Some(x) => x,
        None => "en",
    };
    if language.contains("hu") {
        language = "hu";
    } else {
        language = "en";
    }
    let language = language.to_owned();

    println!("handeling - {}", request.path);
    use Method as M;
    match (&request.method, request.path) {
        (M::GET, "/") => handle_page_return(stream, "200 OK", None, "/index.html"),
        (_, path) if path.contains("/wedding") => wedding::routing(stream, Request { path: path.split_once("/wedding").unwrap().1, .. request }),
        (_, path) if path.contains("/neptunCRF") => neptunCRF::routing(stream, Request { path: path.split_once("/neptunCRF").unwrap().1, .. request }, users),
        (_, "/favicon.ico") => handle_image(stream, "/favicon.ico"),
        (_, _) => vue::routing(stream, request),
        (M::GET, "/vue_test") => handle_page_return(stream, "200 OK", None, "/pages_vue/index.html"),
        (M::GET, "/debug") => handle_debug(stream, request),
        _ => {
            println!("404 Page not found");
            handle_page_return(stream, "404 NOT FOUND", None, &(language + "/404.html"));},
    }
}