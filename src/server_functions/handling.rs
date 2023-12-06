use std::io::{BufReader, BufRead, Error, ErrorKind, BufWriter};

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
    if buffer.len() < 4 {
        return Err(Error::new(ErrorKind::Other, "Error reading request"));
    }
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
    let constant_headers = format!(
        "Server: ligvigfui's rust webserver/{VERSION}\r\n\
        Content-Length: {}\r\n",
        contents.len()
    );
    let headers_together = match headers {
        Some(header_vec) => {
            format!("{}{}", constant_headers, header_vec.join("\r\n"))
        },
        None => constant_headers,
    };
    let response = format!(
        "HTTP/1.1 {status}\r\n\
        {headers_together}\r\n\
        {contents}",
    );
    if crate::DEBUG >= crate::DebugLevel::HIGH {
        if response.len() > crate::DEBUG_LEN {
            println!(
                "Response: \n\
                {}",
                &response[..crate::DEBUG_LEN]
        );}
        else {
            println!(
                "Response: \n\
                {}",
                response
        );}
    }
    if let Err(e) = send_response(stream, &response, None) {
        println!("{}", e);
    }
}

fn default_handle_files(stream: &mut TcpStream, status: &str, headers: Option<Vec<&str>>, contents: Vec<u8>) {
    let chunked = contents.len() > 1024;
    let constant_headers = if chunked {
        "Transfer-Encoding: chunked\r\n".to_string()
    } else {
        format!(
            "Content-Length: {}\r\n",
            contents.len()
        )
    };
    let headers_together = match headers {
        Some(header_vec) => {
            format!("{}{}", constant_headers, header_vec.join("\r\n"))
        },
        None => constant_headers,
    };
    let response = format!(
        "HTTP/1.1 {status}\r\n\
        {headers_together}\r\n\
        \r\n"
    );
    if crate::DEBUG >= crate::DebugLevel::HIGH {
        if response.len() > crate::DEBUG_LEN {
            println!(
                "Response: \n\
                {}",
                &response[..crate::DEBUG_LEN]
        );}
        else {
            println!(
                "Response: \n\
                {}",
                response
        );}
    }
    if chunked {
        if let Err(e) = send_chunked_response(stream, &response, None, Some(contents)) {
            println!("{}", e);
        }
    } else {
        if let Err(e) = send_response(stream, &response, Some(contents)) {
            println!("{}", e);
        }
    }
}

pub fn handle_debug(stream: &mut TcpStream, request: &Request) {
    let request_str = format!("{:?}", request);
    println!("Debug request: {request_str}");
    if let Err(e) = send_response(stream, &format!(
        "HTTP/1.1 200 OK\r\n\
        Content-Length: {}\r\n\
        \r\n\
        {}",
        request_str.len(),
        request_str
    ), None) {
        println!("{}", e);
    }
}

pub fn handle_file(stream: &mut TcpStream, path: &str) {
    match handle_file_inner(stream, format!("pages/assets/{}", path)) {
        Err(e) => {
            println!("{}", e);
        }
        Ok(_) => {}
    }
}

pub fn handle_file_inner(stream: &mut TcpStream, path: String) -> Result<(), io::Error> {
    let mut file = File::open(&path)?;
    let image_format = match path.split(".").last().unwrap() {
        "svg" => "image/svg+xml",
        "exe" => "application/vnd.microsoft.portable-executable",
        "png" => "image/png",
        "gif" => "image/gif",
        "webp" => "image/webp",
        "jpeg" | "jpg" => "image/jpeg",
        format => return Err(Error::new(ErrorKind::Unsupported, format!("format {format} is not jet supported")))
    };
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;

    default_handle_files(
        stream,
        CODE[&200],
        Some(vec![
            format!("Content-Type: {image_format}").as_str(),
        ]),
        contents
    );
    Ok(())
}

fn send_response(stream: &mut TcpStream, response: &str, contents: Option<Vec<u8>>) -> Result<(),  Error> {
    stream.write_all(response.as_bytes())?;
    if contents.is_some() {
        stream.write_all(&contents.unwrap())?;
    }
    print!("\n");
    stream.flush()
}

fn send_chunked_response(stream: &mut TcpStream, headers: &str, body: Option<&str>, contents: Option<Vec<u8>>) -> Result<(),  Error> {
    let mut writer = BufWriter::new(stream);
    writer.write_all(headers.as_bytes())?;
    let chunkable = match (body, contents) {
        (Some(x), None) => x.as_bytes().into(),
        (None, Some(x)) => x,
        _ => panic!("send_chunked_response: You should only provide one option or the other")
    };
    let chunks = chunkable.chunks(8192);
    for chunk in chunks {
        let len = format!("{:X}\r\n", chunk.len());
        let chunk_data = [len.as_bytes(), chunk, b"\r\n"].concat();
        writer.write_all(&chunk_data)?;
    }
    writer.write_all(b"0\r\n\r\n")?;
    writer.flush()
}