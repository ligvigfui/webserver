pub use std::{
    collections::HashMap,
    fs::{self, File},
    fmt::Display,
    io::{self, Read, Write},
    net::TcpStream,
    path::PathBuf,
    sync::{Arc, Mutex},
};

pub use crate::{
    debug::*,
    extensions::*,
    server_functions::{*,
        handling::*,
        headers::{*,
            Header::Host as Host,
            Header::AcceptLanguage as AcceptLanguage,
            Header::ContentLength as ContentLength,
            Header::Cookies as Cookies,
            Header::Connection as Connection,
            Header::ContentType as ContentType,
        },
        http_verison::*,
        method::*,
        request::*,
        response::*,
        response_payload::*,
        routing::*,
        status_code::*,
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

pub static VERSION: &str = "0.3.0";