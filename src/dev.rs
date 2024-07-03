use crate::*;

pub fn routing(request: &Request) -> Response {
    match (&request.method, request.path.as_str()) {
        (GET, "/favicon.ico") => 
            Response::new(Payload::File(PathBuf::from("assets/favicon.ico"))),
        (GET, "/debug") => {
            println!("Debug request: {:?}", request);
            Response::default()
        },
        _ => vue::routing(request),
    }
}