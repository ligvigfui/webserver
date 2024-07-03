use crate::*;

pub fn routing(request: &mut Request, users: Arc<Vec<Mutex<User>>>) -> Response {
    match (&request.method, request.path.as_str()) {
        (GET, "" | "/") => {
            let lang = request.headers.get(&AcceptLanguage).unwrap();
            Response::new(Payload::File(PathBuf::from(format!("pages/{lang}/neptunCRF/neptunCRF.html"))))
        },
        (GET, "/icon") => Response::new(Payload::File(PathBuf::from("assets/neptunCRF/icon.png"))),
        (GET, "/EULA") => Response::new(Payload::File(PathBuf::from("pages/hu/neptunCRF/EULA.html"))),
        (POST, "/login") => handle_neptun_login(request, users),
        (_, path) if path.starts_with("/client") => {
            request.path = request.path.replacen("/client", "", 1);
            client_routing(request)
        },
        _ => Response::_404(request)
    }
}

fn client_routing(request: &Request) -> Response {
    match (&request.method, request.path.as_str()) {
        (GET, "/download") => Response::new(Payload::File(PathBuf::from("neptunCRF/NeptunCRF.exe"))),
        (GET, "/latestversion") => Response::new(Payload::Bites(VERSION.as_bytes().to_vec())),
        _ => Response::_404(request)
    }
}