use crate::*;

use Method as M;

pub fn routing(stream: &mut TcpStream, request: &mut Request, users: Arc<Vec<Mutex<User>>>){
    match (&request.method, request.path.as_str()) {
        (M::GET, "" | "/") => handle_page_return(stream, CODE[&200], None,
        &(format!("{}/neptunCRF/neptunCRF.html", request.headers.get("Accept-Language").unwrap()))),
        (M::GET, "/icon") => handle_file(stream, "neptunCRF/icon.png"),
        (M::GET, "/EULA") => handle_page_return(stream, CODE[&200], None, "hu/neptunCRF/EULA.html"),
        (M::POST, "/login") => handle_neptun_login(stream, request, users),
        (_, path) if path.starts_with("/client") => {
            request.path = request.path.replacen("/client", "", 1);
            client_routing(stream, &request)
        },
        _ => response404(stream, request),
    }
}

fn client_routing(stream: &mut TcpStream, request: &Request) {
    match (&request.method, request.path.as_str()) {
        (M::GET, "/download") => handle_file(stream, "neptunCRF/NeptunCRF.exe"),
        (M::GET, "/latestversion") => default_handle(stream, CODE[&200], None, "0.4.0"),
        _ => response404(stream, request),
    }
}