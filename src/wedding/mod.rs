use crate::*;

pub use request::form::Form;

pub mod request;

pub fn routing(stream: &mut TcpStream, request: &Request) {
    use Method as M;
    match (&request.method, request.path.as_str()) {
        (M::GET, "" | "/") => handle_page_return(stream, CODE[&200], None, "hu/wedding/wedding.html"),
        (M::GET, "/kezdolap.webp") => handle_file(stream, "wedding/kezdolap.webp"),
        (M::GET, "/favicon.gif") => handle_file(stream, "wedding/favicon.gif"),
        (M::POST, "/form") => handle_form(stream, request),
        _ => response404(stream, request),
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