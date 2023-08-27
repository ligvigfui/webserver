use hash::handle_neptun_login_first;
use hash::handle_neptun_login_other;
use webserver::ThreadPool;
use lib::User;
use webserver::extract_anything;
use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::sync::Arc;
use std::sync::Mutex;
use std::vec;
mod lib;
mod hash;


static VERSION: &str = "0.1.1-dev.1";
static DEBUG: bool = false;
fn main() {
    
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
    let pool = ThreadPool::new(4);
    
    // load users from users.json
    let mut users_noarc: Vec<Mutex<User>> = Vec::new();
    let mut users_file = File::open("src/users.json").unwrap();
    let mut contents = String::new();
    users_file.read_to_string(&mut contents).unwrap();
    let users_vec: Vec<User> = serde_json::from_str(&contents).unwrap();
    for user in users_vec {
        users_noarc.push(Mutex::new(user));
    }
    let users = Arc::new(users_noarc);
    println!("Loaded users from users.json");



    // listen for connections
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let users = Arc::clone(&users);
        pool.execute(move || {
            handle_connection(stream, users);
        });
    }



    // write users to users.json in a json format
    let mut file = File::create("users.json").unwrap();
    let users_vec = Arc::try_unwrap(users).unwrap().into_iter().map(|x| x.into_inner().unwrap()).collect::<Vec<User>>();
    file.write_all(serde_json::to_string_pretty(&users_vec).unwrap().as_bytes()).unwrap();
    println!("Saved users to users.json");
    
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
        b"GET /wedding HTTP/1.1" => {
            println!("Handling wedding_test");
            default_handle_page_return(&mut stream, "200 OK", "/hu/wedding/wedding.html");},
        b"GET /wedding/demo_image.jpg HTTP/1.1" => {
            println!("Handling wedding/demo_image.jpg");
            handle_image(&mut stream, "pages/hu/wedding/demo_image.jpg");},
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
    default_handle(stream, status, vec![], &contents);
}

fn default_handle(stream: &mut TcpStream, status: &str, headers: Vec<&str>, contents: &str) {
    if DEBUG {
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
            response.insert_str(pos, &format!("ServerVersion: {}\r\n", VERSION));
        }
    }
    default_handle(stream, &status, vec![], &response);
}

fn handle_debug(stream: &mut TcpStream , buffer: [u8; 1024]){
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    send_response(stream, "HTTP/1.1 200 OK\r\n\r\n");
}

fn handle_image(stream: &mut TcpStream, path: &str) {
    match handle_image_inner(stream, path) {
        Err(e) => {
            println!("{}", e);
        }
        Ok(_) => {}
    }
}

fn handle_image_inner(stream: &mut TcpStream, path: &str) -> Result<(), io::Error> {
    let mut file = File::open(path)?;
    let status = "200 OK";
    let headers = vec!["Content-Type: image/jpeg"];
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

fn update() {
    // git pull

    // cargo build

    // run tests?

    // OK -> cargo build --release
    
    // OK -> restart the webserver service

}
