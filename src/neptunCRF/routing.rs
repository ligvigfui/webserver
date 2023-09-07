use std::net::TcpStream;

use crate::*;

pub fn routing(stream: &mut TcpStream, request: Request, users: Arc<Vec<Mutex<User>>>){
    let mut language = match request.get_header("Accept-Language") {
        Some(x) => x,
        None => "en",
    };
    if language.contains("hu") {
        language = "hu";
    } else {
        language = "en";
    }
    let language = language.to_owned();

    use Method as M;
    match (&request.method, request.path) {
        (M::GET, "") => handle_page_return(stream, "200 OK", None,  &(language + "/neptunCRF/neptunCRF.html")),
        (M::GET, "/icon") => handle_image(stream, "/neptunCRF/icon.png"),
        (M::GET, "/EULA") => handle_page_return(stream, "200 OK", None, &("/hu/neptunCRF/EULA.html")),
        (M::POST, "/login") => handle_neptun_login(stream, request, users),
        _ => {
            println!("404 Page not found");
            handle_page_return(stream, "404 NOT FOUND", None, &(language + "/404.html"));},
    }
}