use crate::*;

pub fn routing(stream: &mut TcpStream, request: &Request) {
    use Method as M;
    match (&request.method, request.path.as_str()) {
        (M::GET, "/favicon.ico") => handle_file(stream, "favicon.ico"),
        (M::GET, "/debug") => handle_debug(stream, request),
        _ => vue::routing(stream, request),
    }
}