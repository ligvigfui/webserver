use std::{ 
    net::TcpListener, 
    sync::Arc,
};

use webserver::{
    self,
    thread::ThreadPool,
    neptunCRF,
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
            webserver::handling::handle_connection(stream, users);
            println!("\x1b[38;5;22mDone with request\x1b[0m");
        });
    }

    neptunCRF::shutdown(neptun_users);
    println!("Shutting down.");
}