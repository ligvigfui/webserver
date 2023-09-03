use std::{
    fs::{self, File},
    io::{Write, Read, self},
    net::TcpStream, 
    ops::Add,
    str, 
};

use chrono::format::format;

use crate::Request;


pub fn handle_page_return(stream: &mut TcpStream, status: &str, html_name: &str) {
    let contents = match fs::read_to_string("pages/".to_owned() + html_name)
    {
        Ok(x) => x,
        Err(e) => {
            println!("Error reading file: {}\n{}", e, html_name);
            String::from("Error reading file")
        }
    };
    default_handle(stream, status, vec![], &contents);
}

pub fn default_handle(stream: &mut TcpStream, status: &str, headers: Vec<&str>, contents: &str) {
    if crate::DEBUG {
        println!("Response: {}", contents);}
    let mut response = format!(
        "HTTP/1.1 {}\r\n",
        status);
    response.push_str(&headers.join("\r\n"));
    response.push_str(&format!(
        "Content-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    ));
    send_response(stream, &response);
}

pub fn handle_debug(stream: &mut TcpStream, request: Request) {
    println!("Debug request: {:?}", request);    
    send_response(stream, "HTTP/1.1 200 OK\r\n\r\n");
}

pub fn handle_image(stream: &mut TcpStream, path: &str) {
    match handle_image_inner(stream, path) {
        Err(e) => {
            println!("{}", e);
        }
        Ok(_) => {}
    }
}

fn handle_image_inner(stream: &mut TcpStream, path: &str) -> Result<(), io::Error> {
    let mut file = File::open(format!("pages/assets{}", &path))?;
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