use std::net::TcpStream;

use crate::*;

pub fn routing(stream: &mut TcpStream, request: Request) {
    use Method as M;
    match (&request.method, request.path) {
        (M::GET, "/favicon.ico") => handle_image(stream, "favicon.ico"),
        (M::GET, "/debug") => handle_debug(stream, request),
        _ => vue::routing(stream, request),
    }
}