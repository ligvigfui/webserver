use std::{
    fs::File,
    net::TcpStream, 
    io::{Write, Read},
    sync::{Arc, Mutex},
    str, 
};
use serde::{Deserialize, Serialize};

use crate::routing::handling::default_handle;
use hash::{handle_neptun_login_first, handle_neptun_login_other};

pub mod hash;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub email: String,
    pub password: String,
    pub MAC: String,
    pub time: u64,
    // server sends even counts
    pub count: i32,
}

impl User {
    pub fn new(email: String, password: String) -> User {
        User {
            email,
            password,
            MAC: String::from(""),
            time: 0,
            count: 1,
        }
    }
}

pub fn neptunCRF_init() -> Arc<Vec<Mutex<User>>> {
    // load users from users.json
    let mut users_noarc: Vec<Mutex<User>> = Vec::new();
    let mut users_file = File::open("src/users.json").unwrap();
    let mut contents = String::new();
    users_file.read_to_string(&mut contents).unwrap();
    let users_vec: Vec<User> = serde_json::from_str(&contents).unwrap();
    for user in users_vec {
        users_noarc.push(Mutex::new(user));
    }
    println!("Loaded users from users.json");
    Arc::new(users_noarc)
}


pub fn handle_neptun_login(stream: &mut TcpStream, buffer: [u8; 1024], users: Arc<Vec<Mutex<User>>>) {
    let (status, mut response);
    let buffer_str = str::from_utf8(&buffer).unwrap();
    if buffer_str.contains("Id: ") {
        (status, response) = handle_neptun_login_first(buffer_str, &users);
    } else {
        (status, response) = handle_neptun_login_other(buffer_str, &users);
    }
    if response.contains("Error") {
        if let Some(pos) = response.rfind("\r\n\r\n") {
            response.insert_str(pos, &format!("ServerVersion: {}\r\n", crate::VERSION));
        }
    }
    default_handle(stream, &status, vec![], &response);
}

pub fn neptunCRF_shutdown(users: Arc<Vec<Mutex<User>>>) {
    // write users to users.json in a json format
    let mut file = File::create("users.json").unwrap();
    let users_vec = Arc::try_unwrap(users).unwrap().into_iter().map(|x| x.into_inner().unwrap()).collect::<Vec<User>>();
    file.write_all(serde_json::to_string_pretty(&users_vec).unwrap().as_bytes()).unwrap();
    println!("Saved users to users.json");
}