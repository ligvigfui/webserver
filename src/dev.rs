use std::net::TcpStream;

use crate::*;

pub fn routing(stream: &mut TcpStream, request: Request) {
    use Method as M;
    match (&request.method, request.path) {
        (_, path) if path.starts_with("/vue") => vue::routing(stream, 
            Request {
                path: &path.replace("/vue", ""),
                ..request
            }),
        (M::GET, "/favicon.ico") => handle_image(stream, "/favicon.ico"),
        (M::GET, "/") => handle_page_return(stream, CODES[&200], None, "/index.html"),
        (M::GET, "/debug") => handle_debug(stream, request),
        _ => response404(stream, request),
    }
}