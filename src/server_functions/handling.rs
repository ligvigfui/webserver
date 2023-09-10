use std::{
    fs::{self, File},
    io::{Write, Read, self},
    net::TcpStream, 
    ops::Add,
    str, 
};

use crate::Request;


pub fn handle_page_return(stream: &mut TcpStream, status: &str, headers: Option<Vec<&str>>, html_name: &str) {
    let contents = match fs::read_to_string("pages/".to_owned() + html_name)
    {
        Ok(x) => x,
        Err(e) => {
            println!("Error reading file: {}\npages/{}", e, html_name);
            format!("{}", e)
        }
    };
    default_handle(stream, status, headers, &contents);
}

pub fn default_handle(stream: &mut TcpStream, status: &str, headers: Option<Vec<&str>>, contents: &str) {
    let mut response = format!(
        "HTTP/1.1 {}\r\n",
        status);
    match headers {
        Some(header_vec) => {
            response.push_str(&header_vec.join("\r\n"));
            response.push_str("\r\n");
        },
        None => {}
    };
    response.push_str(&format!(
        "Content-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    ));
    if crate::DEBUG {
        println!("Response: {}", &response);}
    send_response(stream, &response);
}

pub fn handle_debug(stream: &mut TcpStream, request: Request) {
    println!("Debug request: {:?}", request);    
    send_response(stream, "HTTP/1.1 200 OK\r\n\r\n");
}

pub fn handle_image(stream: &mut TcpStream, path: &str) {
    match handle_image_inner(stream, format!("pages/assets{}", path)) {
        Err(e) => {
            println!("{}", e);
        }
        Ok(_) => {}
    }
}

pub(crate) fn handle_image_inner(stream: &mut TcpStream, path: String) -> Result<(), io::Error> {
    let mut file = File::open(&path)?;
    let status = "200 OK";
    let image_format = path.split(".").last().unwrap();
    let content_type = String::from("Content-Type: image/").add(image_format);
    let headers = vec![content_type.as_str(), "Connection: close"];
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;
    stream.write_all(format!("HTTP/1.1 {}\r\n", status).as_bytes())?;
    stream.write_all(headers.join("\r\n").as_bytes())?;
    stream.write_all(format!("Content-Length: {}\r\n\r\n", contents.len()).as_bytes())?;
    stream.write_all(&contents)?;
    stream.flush()?;
    Ok(())
}

fn send_response(stream: &mut TcpStream, response: &str) {
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    print!("\n");
}