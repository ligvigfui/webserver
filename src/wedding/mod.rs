use std::net::TcpStream;

use crate::*;

pub fn routing(stream: &mut TcpStream, request: Request) {
    use Method as M;   
    match (&request.method, request.path) {
        (M::GET, "") => handle_page_return(stream, "200 OK", None, "/hu/wedding/wedding.html"),
        (M::GET, "/demo_image.jpg") => handle_image(stream, "/wedding/demo_image.jpg"),
        (M::GET, "/form") => handle_debug(stream, request),
        _ => code404(stream, request),
    }
}