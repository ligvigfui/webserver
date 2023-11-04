use std::net::TcpStream;

use crate::*;

pub fn routing(stream: &mut TcpStream, request: Request, users: Arc<Vec<Mutex<User>>>){
    use Method as M;
    match (&request.method, request.path) {
        (M::GET, "") => handle_page_return(stream, CODES[&200], None,
        &(format!("{}/neptunCRF/neptunCRF.html", request.get_header("Accept-Language").unwrap()))),
        (M::GET, "/icon") => handle_image(stream, "neptunCRF/icon.png"),
        (M::GET, "/EULA") => handle_page_return(stream, CODES[&200], None, &("hu/neptunCRF/EULA.html")),
        (M::POST, "/login") => handle_neptun_login(stream, request, users),
        _ => response404(stream, request),
    }
}