use request::form::Form;

use crate::*;

pub mod request;

pub fn routing(request: &Request) -> Response {
    match (&request.method, request.path.as_str()) {
        (GET, "" | "/") => Response::new(Payload::File(PathBuf::from("./pages/hu/wedding/wedding.html"))),
        (GET, image) if image.ends_with(".webp") => Response::new(Payload::File(PathBuf::from(format!("./assets/wedding{image}")))),
        (GET, "/favicon.gif") => Response::new(Payload::File(PathBuf::from("./assets/wedding/favicon.gif"))),
        (GET, "/app.js") => Response::new(Payload::File(PathBuf::from("./pages/hu/wedding/app.js"))),
        (GET, "/style.css") => Response::new(Payload::File(PathBuf::from("./pages/hu/wedding/style.css"))),
        (POST, "/form") => handle_form(request),
        (_, "/debug") => {
            println!("Debug request: {:?}", request);
            Response {
                http_verison: HTTPVerion::_11,
                status: StatusCode::_308,
                headers: HashMap::new(),
                payload: Payload::Redirect("/".to_string()),
            }
        },
        _ => Response::default()
    }
}

fn handle_form<'a>(request: &'a Request) -> Response {
    let form = match request.headers.get(&Header::ContentType) {
        Some(&"application/x-www-form-urlencoded") => {
            let form_hashmap = request.body.split('&').map(|x| x.split('=').collect::<Vec<&str>>()).map(|x| (x[0], x[1])).collect::<HashMap<&str, &str>>();
            Form {
                name: form_hashmap.get("name").unwrap_or(&""),
                email: form_hashmap.get("email").unwrap_or(&""),
                guests: form_hashmap.get("guests").unwrap_or(&"0").parse::<u8>().unwrap_or(0),
            }
        },
        Some(&"application/json") => {
            let form: Result<Form, serde_json::Error> = serde_json::from_str(&request.body);
            match form {
                Ok(form) => form,
                Err(e) => {
                    log_error(e);
                    return Response::_404(request);
                }
            }
        },
        _ => {
            return Response::_404(request);
        }
    };
    println!("{:?}", form);
    Response::default()
}