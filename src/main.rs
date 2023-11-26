use std::{ 
    net::TcpListener, 
    sync::Arc,
};

use webserver::{
    self,
    ThreadPool,
    neptunCRF::{
        self,
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
            webserver::handling::handle_connection(stream, users);
        });
    }

    neptunCRF::shutdown(neptun_users);
    println!("Shutting down.");
}