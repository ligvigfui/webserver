use hash::handle_neptun_login_first;
use hash::handle_neptun_login_other;
use webserver::ThreadPool;
use lib::User;
use webserver::extract_anything;
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
use std::process::{Command, exit};


static VERSION: &str = "0.1.1-dev.1";
fn main() {
    
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
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

    let starts_with = buffer.split(|&x| x == b'\r').next().unwrap();
    let str1 = std::str::from_utf8(&buffer).unwrap();
    let mut language = match extract_anything(str1, "Accept-Language: ") {
        Some(x) => x,
        None => "en".to_owned(),
    };
    if language.contains("hu") {
        language = "hu".to_owned();
    } else {
        language = "en".to_owned();
    }


    match starts_with {
        b"GET / HTTP/1.1" => {
            println!("Handling root");
            default_handle_page_return(&mut stream, "200 OK", &(language + "/hello.html"));},
        b"GET /wedding_test HTTP/1.1" => {
            println!("Handling wedding_test");
            default_handle_page_return(&mut stream, "200 OK", "/hu/wedding_test/wedding.html");},
        b"GET /neptunCRF HTTP/1.1" => {
            println!("Handling neptunCRF");
            default_handle_page_return(&mut stream, "200 OK", &(language + "/neptunCRF.html"));},
        b"GET /neptunCRF/EULA HTTP/1.1" => {
            println!("Handling neptunCRF/EULA");
            default_handle_page_return(&mut stream, "200 OK", &(language + "/neptunCRF/EULA.html"));},
        b"POST /neptunCRF/login HTTP/1.1" => {
            println!("Handling neptunCRF login");
            handle_neptun_login(&mut stream, buffer, users);},
        b"GET /debug HTTP/1.1" => {
            println!("Providing debug info");
            handle_debug(&mut stream, buffer);},
        _ => {
            println!("404");
            default_handle_page_return(&mut stream, "404 NOT FOUND", &(language + "/404.html"));},
    }
}



fn default_handle_page_return(stream: &mut TcpStream, status: &str, html_name: &str){
    let contents = match fs::read_to_string("pages/".to_owned() + html_name)
    {
        Ok(x) => x,
        Err(e) => {
            println!("Error reading file: {}\n{}", e, html_name);
            String::from("Error reading file")
        }
    };
    default_handle(stream, status, &contents);
}

fn default_handle(stream: &mut TcpStream , status: &str, contents: &str) {
    let response = format!(
        "HTTP/1.1 {}\r\nContent-Length: {}\r\n\r\n{}",
        status,
        contents.len(),
        contents
    );
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    print!("\n");
}

fn handle_neptun_login(stream: &mut TcpStream, buffer: [u8; 1024], users: Arc<Vec<Mutex<User>>>) {
    let (status, mut response);
    let buffer_str = std::str::from_utf8(&buffer).unwrap();
    if buffer_str.contains("Id: ") {
        (status, response) = handle_neptun_login_first(buffer_str, &users);
    } else {
        (status, response) = handle_neptun_login_other(buffer_str, &users);
    }
    if response.contains("Error") {
        if let Some(pos) = response.rfind("\r\n\r\n") {
            // Insert the ServerVersion string before the last "\r\n"
            response.insert_str(pos, &format!("ServerVersion: {}\r\n", VERSION));
        }
    }
    default_handle(stream, &status, &response);
}

fn handle_debug(stream: &mut TcpStream , buffer: [u8; 1024]){
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    stream.write_all(b"HTTP/1.1 200 OK\r\n\r\n").unwrap();
    stream.flush().unwrap();
}

fn update() {
    // git pull

    // cargo build

    // run tests?

    // OK -> cargo build --release
    
    // OK -> restart the webserver service

}
