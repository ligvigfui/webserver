use std::net::TcpStream;

use crate::{*, server_functions::{deploy, code404}};

pub fn routing(stream: &mut TcpStream, request: Request) {
    use Method as M;
    match (&request.method, request.path) {
        (_, path) if path.starts_with("/vue") => vue::routing(stream, 
            Request {
                path: &path.replace("/vue", ""),
                ..request
            }),
        (M::GET, "/favicon.ico") => handle_image(stream, "/favicon.ico"),
        (M::GET, "/") => handle_page_return(stream, "200 OK", None, "/index.html"),
        (M::GET, "/debug") => handle_debug(stream, request),
        (M::GET, "/deploy") => deploy(stream),
        _ => code404(stream, request),
    }
}