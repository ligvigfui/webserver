use crate::*;

pub mod request;

pub fn routing(request: &Request) -> Response {
    match (&request.method, request.path.as_str()) {
        (GET, "/wedding.html" | "/") => Response::new(Payload::File(PathBuf::from("./pages/hu/wedding/wedding.html"))),
        (GET, "/bhj.html") => Response::new(Payload::File(PathBuf::from("./pages/hu/wedding/bhj.html"))),
        (GET, image) if image.ends_with(".webp") => Response::new(Payload::File(PathBuf::from(format!("./assets/wedding{image}")))),
        (GET, "/favicon.gif") => Response::new(Payload::File(PathBuf::from("./assets/wedding/favicon.gif"))),
        (GET, "/app.js") => Response::new(Payload::File(PathBuf::from("./pages/hu/wedding/app.js"))),
        (GET, "/style.css") => Response::new(Payload::File(PathBuf::from("./pages/hu/wedding/style.css"))),
        (_, "/debug") => {
            println!("Debug request: {:?}", request);
            Response {
                http_verison: HTTPVerion::_11,
                status: StatusCode::_308,
                headers: HashMap::new(),
                payload: Payload::Redirect("/".to_string()),
            }
        },
        _ => Response::_404(request)
    }
}