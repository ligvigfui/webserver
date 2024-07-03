use crate::*;

pub fn routing(request: &Request) -> Response {
    match (&request.method, request.path.as_str()) {
        (GET, "" | "/") => Response::new(Payload::File(PathBuf::from("vue/dist/index.html"))),
        (GET, path) => match path.split(".").last() {
            Some("png") | Some("jpg") | Some("jpeg") | Some("gif") | Some("svg") =>
                Response::new(Payload::File(PathBuf::from(format!("vue/dist/{}", request.path)))),
            None | Some(_) =>
                Response::new(Payload::File(PathBuf::from(format!("vue/{}", request.path)))),
        },
        _ => Response::_404(request)
    }
}