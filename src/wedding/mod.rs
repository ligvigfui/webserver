use crate::*;

pub fn routing(stream: &mut TcpStream, request: &Request) {
    use Method as M;
    match (&request.method, request.path.as_str()) {
        (M::GET, "" | "/") => handle_page_return(stream, CODE[&200], None, "hu/wedding/wedding.html"),
        (M::GET, "/kezdolap.webp") => handle_file(stream, "wedding/kezdolap.webp"),
        (M::GET, "/favicon.gif") => handle_file(stream, "wedding/favicon.gif"),
        (M::POST, "/form") => handle_debug(stream, request),
        _ => response404(stream, request),
    }
}