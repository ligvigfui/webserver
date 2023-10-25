use crate::*;

pub mod routing;
pub mod handling;
pub mod status_codes;
pub mod method;
pub mod request;

pub fn response404(stream: &mut std::net::TcpStream, request: Request) {
    println!("404 Page not found");
    handle_page_return(stream, "404 NOT FOUND", None,
    &(format!("{}/404.html", request.get_header("Accept-Language").unwrap())));
}