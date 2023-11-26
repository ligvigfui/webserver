use crate::*;

pub mod routing;
pub mod handling;
pub mod status_codes;
pub mod method;
pub mod request;

pub fn response404(stream: &mut std::net::TcpStream, request: &Request) {
    let host = match request.headers.get("Host") {
        Some(x) => x,
        None => "noHost",
    };
    let accept_language = match request.headers.get("Accept-Language") {
        Some(x) => x,
        None => "en",
    };
    println!("Error {} - Requested page: {}{}", CODES[&404], host, &request.path);
    handle_page_return(stream, CODES[&404], None,
    &(format!("{}/404.html", accept_language)));
}