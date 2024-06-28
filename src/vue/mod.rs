use crate::*;

pub fn routing(request: &Request) -> Response {
    use Method as M;
    match (&request.method, request.path.as_str()) {
        (M::GET, "" | "/") => Response::new(ResponsePayload::File(PathBuf::from("vue/dist/index.html"))),
        (M::GET, path) => match path.split(".").last() {
            Some("png") | Some("jpg") | Some("jpeg") | Some("gif") | Some("svg") =>
                Response::new(ResponsePayload::File(PathBuf::from(format!("vue/dist/{}", request.path)))),
            None | Some(_) =>
                Response::new(ResponsePayload::File(PathBuf::from(format!("vue/{}", request.path)))),
        },
        _ => Response::_404(request)
    }
}