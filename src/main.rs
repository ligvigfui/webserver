use std::{ 
    io::prelude::*,
    net::{
        TcpListener, 
        TcpStream}, 
    sync::{
        Arc, 
        Mutex}};

use webserver::{
    self,
    ThreadPool,
    Request,
    routing::routing::*,
    neptunCRF::{
        self,
        User,
    },
};

fn main() {
    
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
    let pool = ThreadPool::new(4);

    let neptun_users = neptunCRF::init();

    // listen for connections
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let users = Arc::clone(&neptun_users);
        pool.execute(move || {
            handle_connection(stream, users);
        });
    }

    neptunCRF::shutdown(neptun_users);
    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream, users: Arc<Vec<Mutex<User>>>) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let request = Request::from(&buffer);
    
    routing(&mut stream, request, users);
}