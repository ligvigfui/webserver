use std::{
    fs::File,
    io::{Write, Read},
    str, 
};
use serde::{Deserialize, Serialize};

use crate::*;

pub mod hash;
pub mod routing;
pub use routing::routing as routing;

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

pub fn init() -> Arc<Vec<Mutex<User>>> {
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

pub fn shutdown(users: Arc<Vec<Mutex<User>>>) {
    // write users to users.json in a json format
    let mut file = File::create("users.json").unwrap();
    let users_vec = Arc::try_unwrap(users).unwrap().into_iter().map(|x| x.into_inner().unwrap()).collect::<Vec<User>>();
    file.write_all(serde_json::to_string_pretty(&users_vec).unwrap().as_bytes()).unwrap();
    println!("Saved users to users.json");
}