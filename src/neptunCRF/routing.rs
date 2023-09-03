use std::net::TcpStream;

use crate::*;

pub fn routing(stream: &mut TcpStream, request: Request, users: Arc<Vec<Mutex<User>>>){
    let path = request.path.split("/neptunCRF").next().unwrap();
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
    match (&request.method, path) {
        (M::GET, "/neptunCRF") => handle_page_return(stream, "200 OK", &(language + "/neptunCRF/neptunCRF.html")),
        (M::GET, "/neptunCRF/icon") => handle_image(stream, "pages/assets/neptunCRF/icon.png"),
        (M::GET, "/neptunCRF/EULA") => handle_page_return(stream, "200 OK", &("/hu/neptunCRF/EULA.html")),
        (M::POST, "/neptunCRF/login") => handle_neptun_login(stream, request, users),
        _ => {
            println!("404 Page not found");
            handle_page_return(stream, "404 NOT FOUND", &(language + "/404.html"));},
    }
}