pub use std::{
    collections::HashMap,
    fmt::Display,
    fs::{self, File},
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
            Header::AcceptLanguage as AcceptLanguage,
            Header::Connection as Connection,
            Header::ContentLength as ContentLength,
            Header::ContentType as ContentType,
            Header::Cookies as Cookies,
            Header::Host as Host,
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

pub mod dev;
pub mod extensions;
#[allow(non_snake_case)]
pub mod neptunCRF;
pub mod server_functions;
pub mod vue;
pub mod wedding;

pub static VERSION: &str = "0.3.0";