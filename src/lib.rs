pub use std::{
    collections::HashMap,
    fs::{self, File},
    io::{self, Read, Write},
    net::TcpStream,
    sync::{Arc, Mutex},
};

pub use crate::{
    debug::*,
    extensions::*,
    server_functions::{*,
        routing::*,
        handling::*,
        status_codes::*,
        method::*,
        request::*,
    },
    neptunCRF::{User, hash::handle_neptun_login}
};

pub mod server_functions;
#[allow(non_snake_case)]
pub mod neptunCRF;
pub mod wedding;
pub mod dev;
pub mod vue;
pub mod extensions;

pub static VERSION: &str = "0.2.1";