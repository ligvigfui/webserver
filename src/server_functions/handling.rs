use std::{io::{BufReader, BufRead, Error, ErrorKind, BufWriter}, net::Shutdown};

use crate::*;

pub fn handle_connection(mut stream: TcpStream, users: Arc<Vec<Mutex<User>>>) {
    let buffer = match read_to_buffer(&mut stream) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };
    let mut request = match Request::try_from(&buffer) {
        Ok(x) => x,
        Err(e) => {
            eprint!("Error parsing request: {:?}", e);
            if DEBUG == DebugLevel::HIGH {
                if buffer.len() > DEBUG_LEN {
                    eprint!(": {:?}", &buffer[..DEBUG_LEN]);}
                else {eprint!(": {:?}", &buffer);}
            }
            eprintln!();
            return;
        }
    };
    
    let mut response = routing(&mut request, users);
    match send_response(&mut stream, &mut response) {
        Ok(_) => {},
        Err(e) => eprintln!("{}", e),
    }
}

fn read_to_buffer(stream: &mut TcpStream) -> Result<String, Error> {
    let mut reader = BufReader::new(stream);
    let mut buffer: String = String::new();
    reader.read_line(&mut buffer)?;
    if buffer.len() < 4 {
        return Err(Error::new(ErrorKind::InvalidInput, "Error reading request"));
    }
    while &buffer[buffer.len()-4..] != "\r\n\r\n" {
        reader.read_line(&mut buffer)?;
    }
    if let Some((_, content_len)) = buffer.split_once("Content-Length:") {
        let content_length = match content_len.trim().split("\r\n").next() {
            Some(x) => x,
            None => return Err(Error::new(ErrorKind::InvalidData, "Error parsing content length"))
        };

        let content_length = match content_length.parse() {
            Ok(x) => x,
            Err(e) => return Err(Error::new(ErrorKind::InvalidData, e))
        };
        let mut content_buffer = vec![0; content_length];
        reader.read_exact(&mut content_buffer)?;
        
        let content_buffer = match String::from_utf8(content_buffer) {
            Ok(x) => x,
            Err(e) => return Err(Error::new(ErrorKind::InvalidData, e))
        };
        buffer.push_str(&content_buffer);
    }
    Ok(buffer)
}

fn send_response(stream: &mut TcpStream, response: &mut Response) -> Result<(), Error> {
    let mut writer = BufWriter::new(stream);
    for bites in response.as_bytes() {
        writer.write_all(&bites)?;
    }
    writer.flush()?;
    writer.into_inner()?.shutdown(Shutdown::Both)
}