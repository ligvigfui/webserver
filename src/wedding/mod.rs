use crate::*;

pub use request::form::Form;

pub mod request;

pub fn routing(request: &Request) -> Response {
    use Method as M;
    match (&request.method, request.path.as_str()) {
        (M::GET, "" | "/") => Response::new(ResponsePayload::File(PathBuf::from("./pages/hu/wedding/wedding.html"))),
        (M::GET, image) if image.ends_with(".webp") => Response::new(ResponsePayload::File(PathBuf::from(format!("./assets/wedding{image}")))),
        (M::GET, "/favicon.gif") => Response::new(ResponsePayload::File(PathBuf::from("./assets/wedding/favicon.gif"))),
        (M::GET, "/app.js") => Response::new(ResponsePayload::File(PathBuf::from("./pages/hu/wedding/app.js"))),
        (M::GET, "/style.css") => Response::new(ResponsePayload::File(PathBuf::from("./pages/hu/wedding/style.css"))),
        (M::POST, "/form") => handle_form(request),
        _ => Response::default()
    }
}

fn handle_form<'a>(request: &'a Request) -> Response {
    let form: Result<Form<'a>, serde_json::Error> = request.to_form();
    match form {
        Ok(form) => {
            println!("{:?}", form);
            Response {
                http_verison: HTTPVerion::_11,
                status: StatusCode::_200,
                headers: HashMap::from([(Header::AccessControlAllowOrigin, "*".to_string())]),
                payload: ResponsePayload::Json("{\"status\": \"ok\"}".to_owned()),
            }
        },
        Err(e) => {
            println!("{}", e);
            Response::_404(request)
        }
    }
}