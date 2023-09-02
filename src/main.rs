use lib::User;
use std::{
    fs::{
        self, 
        File}, 
    io::{
        self, 
        prelude::*}, 
    net::{
        TcpListener, 
        TcpStream}, 
    ops::Add, 
    sync::{
        Arc, 
        Mutex}, 
    vec};

use webserver::ThreadPool;
use webserver::extract_anything;

pub mod lib;


fn main() {
    
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
    let pool = ThreadPool::new(4);

    let neptun_users = neptunCRF_init();

    // listen for connections
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let users = Arc::clone(&neptun_users);
        pool.execute(move || {
            handle_connection(stream, users);
        });
    }

    neptunCRF_shutdown(&neptun_users);
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
        b"POST /wedding/form HTTP/1.1" => {
            println!("Handling wedding/form");
            handle_debug(&mut stream, buffer);},
        b"GET /vue_test HTTP/1.1" => {
            println!("Handling vue_test");
            default_handle_page_return(&mut stream, "200 OK", "/pages_vue/index.html");},
        b"GET /neptunCRF HTTP/1.1" => {
            println!("Handling neptunCRF");
            default_handle_page_return(&mut stream, "200 OK", &(language + "/neptunCRF/neptunCRF.html"));},
        b"GET /neptunCRF/icon HTTP/1.1" => {
            println!("Handling neptunCRF icon");
            handle_image(&mut stream, "pages/assets/neptunCRF/icon.png");},
        b"GET /neptunCRF/EULA HTTP/1.1" => {
            println!("Handling neptunCRF/EULA");
            default_handle_page_return(&mut stream, "200 OK", &("/hu/neptunCRF/EULA.html"));},
        b"POST /neptunCRF/login HTTP/1.1" => {
            println!("Handling neptunCRF login");
            handle_neptun_login(&mut stream, buffer, users);},
        b"GET /debug HTTP/1.1" => {
            handle_debug(&mut stream, buffer);},
        _ => {
            println!("404 - {}", std::str::from_utf8(&starts_with).unwrap());
            default_handle_page_return(&mut stream, "404 NOT FOUND", &(language + "/404.html"));},
    }
}




fn update() {
    // git pull

    // cargo build

    // run tests?

    // OK -> cargo build --release
    
    // OK -> restart the webserver service

}
