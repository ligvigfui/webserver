use hash::get_response;
use webserver::ThreadPool;
use lib::User;
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

    match starts_with {
        b"GET / HTTP/1.1" => {
            println!("Handling root");
            default_handle_page_return(&mut stream, "200 OK", "hello.html");},
        b"GET /sleep HTTP/1.1" => {
            println!("Sleeping for 5 seconds");
            thread::sleep(Duration::from_secs(5));
            default_handle_page_return(&mut stream, "200 OK", "hello.html");},
        b"GET /neptun_fos HTTP/1.1" => {
            println!("Handling neptun_fos");
            default_handle_page_return(&mut stream, "200 OK", "neptun_fos.html");},
        b"POST /neptun_fos/login HTTP/1.1" => {
            println!("Handling neptun_fos login");
            handle_neptun_fos_login(&mut stream, buffer, users);},
        b"GET /debug HTTP/1.1" => {
            println!("Providing debug info");
            handle_debug(&mut stream, buffer);},
        _ => {
            println!("404");
            default_handle_page_return(&mut stream, "404 NOT FOUND", "404.html");}
    }
}

fn default_handle_page_return(stream: &mut TcpStream, status: &str, html_name: &str){
    let contents = fs::read_to_string("pages/".to_owned() + html_name).unwrap();
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
}

fn handle_neptun_fos_login(stream: &mut TcpStream , buffer: [u8; 1024], users: Arc<Vec<Mutex<User>>>){

}

fn handle_debug(stream: &mut TcpStream , buffer: [u8; 1024]){
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    stream.write_all(b"HTTP/1.1 200 OK\r\n\r\n").unwrap();
    stream.flush().unwrap();
}

fn update() {
    // Clone the repository
    let repo_url = "https://github.com/ligvigfui/repo.git";
    let repo_dir = "repo";
    if let Err(err) = Command::new("git").args(&["clone", repo_url, repo_dir]).status() {
        eprintln!("Failed to clone repository: {}", err);
    }

    // Rename the old executable
    let old_exe_path = std::env::current_exe().unwrap();
    let old_exe_name = old_exe_path.file_name().unwrap();
    let old_exe_backup = old_exe_path.with_file_name(format!("{}_old{}", old_exe_name.to_str().unwrap(), old_exe_path.extension().unwrap_or_default().to_str().unwrap()));
    if let Err(err) = std::fs::rename(&old_exe_path, &old_exe_backup) {
        eprintln!("Failed to rename old executable: {}", err);
        exit(1);
    }

    // Build the latest executable
    if let Err(err) = Command::new("cargo").args(&["build", "--release"]).current_dir(repo_dir).status() {
        eprintln!("Failed to build latest executable: {}", err);
        exit(1);
    }

    // Run the latest executable
    let new_exe_path = format!("{}/target/release/main", repo_dir);
    if let Err(err) = Command::new(&new_exe_path).spawn() {
        eprintln!("Failed to run latest executable: {}", err);
        exit(1);
    }

    // Close this instance of the program
    exit(0);
}
