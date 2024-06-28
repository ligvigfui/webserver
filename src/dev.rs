use crate::*;

pub fn routing(request: &Request) -> Response {
    use Method as M;
    match (&request.method, request.path.as_str()) {
        (M::GET, "/favicon.ico") => 
            Response::new(ResponsePayload::File(PathBuf::from("assets/favicon.ico"))),
        (M::GET, "/debug") => {
            println!("Debug request: {:?}", request);
            Response::default()
        },
        _ => vue::routing(request),
    }
}