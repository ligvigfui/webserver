use crate::*;

pub fn routing(stream: &mut TcpStream, request: &Request, users: Arc<Vec<Mutex<User>>>){
    use Method as M;
    if DEBUG >= DebugLevel::LOW {
        println!("Request.path: {}, Request.method: {:?}", request.path, request.method);
    }
    match (&request.method, request.path.as_str()) {
        (M::GET, "" | "/") => handle_page_return(stream, CODE[&200], None,
        &(format!("{}/neptunCRF/neptunCRF.html", request.headers.get("Accept-Language").unwrap()))),
        (M::GET, "/icon") => handle_file(stream, "neptunCRF/icon.png"),
        (M::GET, "/EULA") => handle_page_return(stream, CODE[&200], None, &("hu/neptunCRF/EULA.html")),
        (M::POST, "/login") => handle_neptun_login(stream, request, users),
        (M::GET, "/download") => handle_file(stream, "neptunCRF/NeptunCRF 0.3.0.exe"),
        _ => response404(stream, request),
    }
}