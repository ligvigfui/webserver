use std::io::{BufReader, BufRead, Error, ErrorKind};

use crate::*;

pub fn handle_connection(mut stream: TcpStream, users: Arc<Vec<Mutex<User>>>) {
    let buffer = match read_to_buffer(&mut stream) {
        Ok(x) => x,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
    let mut request = match Request::from(&buffer) {
        Some(x) => x,
        None => {
            print!("Error parsing request");
            if DEBUG == DebugLevel::HIGH {
                if buffer.len() > DEBUG_LEN {
                    print!(": {:?}", &buffer[..DEBUG_LEN]);}
                else {print!(": {:?}", &buffer);}
            }
            println!();
            return;
        }
    };
    
    routing(&mut stream, &mut request, users);
}

fn read_to_buffer(stream: &mut TcpStream) -> Result<String, Error> {
    let mut reader = BufReader::new(stream);
    let mut buffer: String = String::new();
    reader.read_line(&mut buffer)?;
    while &buffer[buffer.len()-4..] != "\r\n\r\n" {
        reader.read_line(&mut buffer)?;
    }
    if let Some(content_len) = buffer.split("Content-Length: ").nth(1) {

        let content_length = match content_len.split("\r\n").next() {
            Some(x) => x,
            None => return Err(Error::new(ErrorKind::Other, "Error parsing content length"))
        }.parse();

        let content_length = match content_length {
            Ok(x) => x,
            Err(e) => return Err(Error::new(ErrorKind::Other, e))
        };
        let mut content_buffer = vec![0; content_length];
        reader.read_exact(&mut content_buffer)?;
        
        let content_buffer = match String::from_utf8(content_buffer) {
            Ok(x) => x,
            Err(e) => return Err(Error::new(ErrorKind::Other, e))
        };
        buffer.push_str(&content_buffer);
    }
    Ok(buffer)
}

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
    let constant_headers = format!("Server: ligvigfui's rust webserver/{VERSION}\r\nContent-Length: {}\r\n",
        contents.len());
    let headers_together = match headers {
        Some(header_vec) => {
            format!("{}{}", constant_headers, header_vec.join("\r\n"))
        },
        None => constant_headers,
    };
    let response = format!(
        "HTTP/1.1 {status}\r\n{headers_together}\r\n{contents}",
    );
    if crate::DEBUG >= crate::DebugLevel::HIGH {
        if response.len() > crate::DEBUG_LEN {
            println!("Response: {}", &response[..crate::DEBUG_LEN]);}
        else {println!("Response: {}", response);}
    }
    send_response(stream, &response);
}

pub fn handle_debug(stream: &mut TcpStream, request: &Request) {
    println!("Debug request: {:?}", request);    
    send_response(stream, "HTTP/1.1 200 OK\r\n\r\n");
}

pub fn handle_image(stream: &mut TcpStream, path: &str) {
    match handle_image_inner(stream, format!("pages/assets/{}", path)) {
        Err(e) => {
            println!("{}", e);
        }
        Ok(_) => {}
    }
}

pub fn handle_image_inner(stream: &mut TcpStream, path: String) -> Result<(), io::Error> {
    let mut file = File::open(&path)?;
    let status = CODES[&200];
    let mut image_format = path.split(".").last().unwrap();
    if image_format == "svg" {
        image_format = "svg+xml";
    }
    let content_type = format!("Content-Type: image/{image_format}");
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


