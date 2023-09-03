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
        (M::GET, "/") => {
            println!("Handling root");
            handle_page_return(stream, "200 OK", &(language + "/hello.html"));},
        (M::GET, "/wedding") => {
            println!("Handling wedding_test");
            handle_page_return(stream, "200 OK", "/hu/wedding/wedding.html");},
        (M::GET, "/wedding/demo_image.jpg") => {
            println!("Handling wedding/demo_image.jpg");
            handle_image(stream, "pages/hu/wedding/demo_image.jpg");},
        (M::GET, "/wedding/form") => {
            println!("Handling wedding/form");
            handle_debug(stream, request);},
        (M::GET, "/vue_test") => {
            println!("Handling vue_test");
            handle_page_return(stream, "200 OK", "/pages_vue/index.html");},
        (M::GET, "/neptunCRF") => {
            println!("Handling neptunCRF");
            handle_page_return(stream, "200 OK", &(language + "/neptunCRF/neptunCRF.html"));},
        (M::GET, "/neptunCRF/icon") => {
            println!("Handling neptunCRF icon");
            handle_image(stream, "pages/assets/neptunCRF/icon.png");},
        (M::GET, "/neptunCRF/EULA") => {
            println!("Handling neptunCRF/EULA");
            handle_page_return(stream, "200 OK", &("/hu/neptunCRF/EULA.html"));},
        (M::POST, "/neptunCRF/login") => {
            println!("Handling neptunCRF login");
            handle_neptun_login(stream, request, users);},
        (M::GET, "/debug") => {
            handle_debug(stream, request);},
        _ => {
            println!("404 - {}", request.path);
            handle_page_return(stream, "404 NOT FOUND", &(language + "/404.html"));},
    }
}