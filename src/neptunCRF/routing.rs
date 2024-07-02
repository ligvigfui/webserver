use crate::*;

pub fn routing(request: &mut Request, users: Arc<Vec<Mutex<User>>>) -> Response {
    match (&request.method, request.path.as_str()) {
        (GET, "" | "/") => {
            let lang = request.headers.get(&AcceptLanguage).unwrap();
            Response::new(ResponsePayload::File(PathBuf::from(format!("pages/{lang}/neptunCRF/neptunCRF.html"))))
        },
        (GET, "/icon") => Response::new(ResponsePayload::File(PathBuf::from("assets/neptunCRF/icon.png"))),
        (GET, "/EULA") => Response::new(ResponsePayload::File(PathBuf::from("pages/hu/neptunCRF/EULA.html"))),
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
        (GET, "/download") => Response::new(ResponsePayload::File(PathBuf::from("neptunCRF/NeptunCRF.exe"))),
        (GET, "/latestversion") => Response::new(ResponsePayload::Bites(VERSION.as_bytes().to_vec())),
        _ => Response::_404(request)
    }
}