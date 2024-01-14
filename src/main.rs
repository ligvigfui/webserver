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
        let stream = match stream {
            Ok(x) => x,
            Err(e) => {
                println!("\x1b[38;5;9mmain/Error: {}\x1b[0m", e);
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