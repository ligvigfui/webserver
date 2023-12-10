use crate::*;
use server_functions as SF;

pub fn routing(stream: &mut TcpStream, request: &Request){
    use Method as M;
    match (&request.method, request.path.as_str()) {
        (M::GET, "" | "/") => handle_page_return(stream, CODE[&200], Some(vec![]), "vue/dist/index.html"),
        (M::GET, path) => match path.split(".").last() {
            Some("png") | Some("jpg") | Some("jpeg") | Some("gif") | Some("svg") => {
                handle_image(stream, path);},
            Some("css") | Some("html") => {
                handle_page_return(stream, CODE[&200], Some(vec![]), &format!("vue/{}", request.path))},
            Some("js") => {
                SF::handling::handle_page_return(stream,
                CODE[&200], Some(vec!["Content-type: text/javascript; charset=UTF-8"]), &format!("vue/{}", request.path))},
            None | Some(_) => {
                handle_page_return(stream, CODE[&200], None, &format!("vue/{}", request.path))}
        },
        _ => response404(stream, request),
    }
}

fn handle_image(stream: &mut TcpStream, path: &str) {
    if DEBUG >= DebugLevel::HIGH {
        println!("Image request: pages/vue{}", path);
    }
    match handle_file_inner(stream, format!("pages/vue{}", path)) {
        Err(e) => {
            println!("{}", e);
        }
        Ok(_) => {}
    }
}

fn handle_page_return(stream: &mut TcpStream, status: &str, headers: Option<Vec<&str>>, path: &str) {
    let extension = path.split(".").last().unwrap();
    let mut headers2 = headers.unwrap();
    let string = format!("Content-type: text/{}", extension);
    headers2.push(&string);
    SF::handling::handle_page_return(stream, status, Some(headers2), path);
}