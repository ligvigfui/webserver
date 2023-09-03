use std::net::TcpStream;

use crate::*;

pub fn routing(stream: &mut TcpStream, request: Request) {
    use Method as M;   
    match (&request.method, request.path) {
        (M::GET, "/wedding") => handle_page_return(stream, "200 OK", "/hu/wedding/wedding.html"),
        (M::GET, "/wedding/demo_image.jpg") => handle_image(stream, "pages/hu/wedding/demo_image.jpg"),
        (M::GET, "/wedding/form") => handle_debug(stream, request),
        _ => {
            println!("404 Page not found");
            handle_page_return(stream, "404 NOT FOUND", "/hu/404.html");},
    }
}