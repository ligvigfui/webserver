use crate::*;

pub mod routing;
pub mod handling;
pub mod status_codes;
pub mod method;
pub mod request;

pub fn response404(stream: &mut std::net::TcpStream, request: Request) {
    println!("Requested page: {}{}\nError {}", &request.get_header("host").unwrap(), &request.path, CODES[&404]);
    handle_page_return(stream, CODES[&404], None,
    &(format!("{}/404.html", request.get_header("Accept-Language").unwrap())));
}