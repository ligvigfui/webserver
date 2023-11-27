use crate::*;

use phf::phf_map;

pub static CODES: phf::Map<u16, &'static str> = phf_map! {
    200u16 => "200 OK",
    400u16 => "400 Bad Request",
    404u16 => "404 Not Found",
    500u16 => "500 Internal Server Error",
    501u16 => "501 Not Implemented",
    505u16 => "505 HTTP Version Not Supported",
};

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