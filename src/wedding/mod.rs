use crate::*;

pub use request::form::Form;

pub mod request;

pub fn routing(request: &Request) -> Response {
    use Method as M;
    match (&request.method, request.path.as_str()) {
        (M::GET, "" | "/") => Response::new(ResponsePayload::File(Path::new("./pages/hu/wedding/wedding.html"))),
        (M::GET, image) if image.ends_with(".webp") => Response::new(ResponsePayload::File(Path::new(&format!("./assets/wedding{image}")))),
        (M::GET, "/favicon.gif") => Response::new(ResponsePayload::File(Path::new("./assets/wedding/favicon.gif"))),
        (M::GET, "/app.js") => Response::new(ResponsePayload::File(Path::new("./pages/hu/wedding/app.js"))),
        (M::GET, "/style.css") => Response::new(ResponsePayload::File(Path::new("./pages/hu/wedding/style.css"))),
        (M::POST, "/form") => handle_form(stream, request),
        _ => Response::default()
    }
}

fn handle_form<'a>(stream: &mut TcpStream, request: &'a Request) {
    let form: Result<Form<'a>, serde_json::Error> = request.to_form();
    match form {
        Ok(form) => {
            println!("{:?}", form);
            default_handle(
                stream,
                CODE[&200],
                Some(vec!["Access-Control-Allow-Origin: *"]),
                "{\"status\": \"ok\"}",
            )
        },
        Err(e) => {
            println!("{}", e);
            response404(stream, request)
        }
    }
}

pub fn handle_file_pages(stream: &mut TcpStream, path: &str) {
    match handle_file_inner(stream, format!("pages/{}", path)) {
        Ok(_) => {}
        Err(e) => {
            println!("{}", e);
        }
    }
}