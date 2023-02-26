use hash::get_response;
use webserver::ThreadPool;
use crate::lib::User;
use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
mod lib;
mod hash;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);
    //todo load in users
    let users = Arc::new(vec![
        Mutex::new(User::new(String::from("ligvigfui@gmail.com"), String::from("hali0123")))
    ]);


    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let users = Arc::clone(&users);
        pool.execute(move || {
            handle_connection(stream, users);
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream, users: Arc<Vec<Mutex<User>>>) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    let neptun = b"GET /neptun_fos HTTP/1.1\r\n";
    let login_neptun = b"PUT /login_neptun_fos HTTP/1.1\r\n";


    if buffer.starts_with(login_neptun) {
        let (status, string) = handle_login(buffer, users);
        let response = format!(
            "HTTP/1.1 {}\r\nContent-Length: {}\r\n\r\n{}",
            status,
            string.len(),
            string
        );
        stream.write_all(response.as_bytes()).unwrap();
        stream.flush().unwrap();
        return;
    } 
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(neptun) {
        ("HTTP/1.1 200 OK", "neptun_fos.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string("pages\\".to_owned() + filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn handle_login(buffer: [u8; 1024], users: Arc<Vec<Mutex<User>>>) -> (String, String){
    //return: status, response
    // convert the buffer to a string
    let string = match String::from_utf8(buffer.to_vec()) {
        Ok(v) => v,
        Err(e) => {
            println!("Invalid UTF-8 sequence: {}", e);
            return ("400 Bad Request".to_string(), "Invalid UTF-8 sequence".to_string());
        }
    };



    println!("Users: {:?}", users);
    
    // do something with the credentials
    get_response(&string, users)
    
}