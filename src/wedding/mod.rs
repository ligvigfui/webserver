use crate::*;

pub fn routing(stream: &mut TcpStream, request: &Request) {
    use Method as M;
    if DEBUG >= DebugLevel::LOW {
        println!("handeling - {}", request.path);
    }
    match (&request.method, request.path.as_str()) {
        (M::GET, "" | "/") => handle_page_return(stream, CODES[&200], None, "hu/wedding/wedding.html"),
        (M::GET, "/demo_image.jpg") => handle_image(stream, "wedding/demo_image.jpg"),
        (M::POST, "/form") => handle_debug(stream, request),
        _ => response404(stream, request),
    }
}