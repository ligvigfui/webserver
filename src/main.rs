use std::{ 
    net::TcpListener, 
    sync::Arc,
};

use webserver::{
    self, log_error, neptunCRF, thread::ThreadPool
};

fn main() {
    println!("Starting server.");
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
    let pool = ThreadPool::new(4);

    let neptun_users = neptunCRF::init();

    // listen for connections
    for stream in listener.incoming() {
        let stream = match stream {
            Ok(x) => x,
            Err(e) => {
                log_error(e);
                continue;
            }
        };
        let users = Arc::clone(&neptun_users);
        pool.execute(move || {
            webserver::handling::handle_connection(stream, users);
        });
    }

    neptunCRF::shutdown(neptun_users);
    println!("Shutting down.");
}