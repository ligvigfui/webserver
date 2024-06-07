use crate::*;

pub fn routing(stream: &mut TcpStream, request: &mut Request, users: Arc<Vec<Mutex<User>>>) {
    /*if request.protocol != "HTTP/1.1" {
        println!("Protocol not supported: {}", request.protocol);
        handle_page_return(stream, CODES[&505], None, "505.html");
    }*/
    match request.headers.get("Accept-Language") {
        Some(x) => match x.contains("hu") {
            true => *request.headers.get_mut("Accept-Language").unwrap() = "hu",
            false => *request.headers.get_mut("Accept-Language").unwrap() = "en",
        },
        None => {
            request.headers.insert("Accept-Language", "en");
        },
    };
    if DEBUG >= DebugLevel::LOW {
        println!("\x1b[38;5;11m{}\x1b[0m", chrono::Local::now().format("%Y-%m-%d %H:%M:%S"));
        println!("\x1b[38;5;21mHandling - Request.path: {}, Request.method: {:?}\x1b[0m", request.path, request.method);
    }
    /*
    let host = match request.headers.get("Host") {
        Some(x) => x,
        None => {
            println!("Did not find host: \"{:?}\"", request.headers.get("Host"));
            handle_page_return(stream, CODE[&400], None, "en/host_not_provided.html");
            return;
        }
    };
    */
    wedding::routing(stream, request);
    /*
    match host.split(":").next().unwrap() {
        "nikiesboldi.ddnsfree.com" |
            "rust-webserver.azurewebsites.net" |
            "192.168.0.10" |
            "nikiesboldi" |
            "homenikiesboldi" => wedding::routing(stream, request),
        "neptuncrf.freeddns.org" |
            "neptuncrf" |
            "homeneptuncrf" => neptunCRF::routing(stream, request, users),
        "coder.ddnsfree.com" | 
            "coder" | 
            "homecoder" => dev::routing(stream, request),
        _ => {
            println!("Did not find host: \"{:?}\"", request.headers.get("Host"));
            response404(stream, request);
        }
    } */
    println!("\x1b[38;5;22mDone with request: {}\x1b[0m", request.path);
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_routing() {
        let request = Request {
            method: Method::GET,
            path: "/",
            headers: [
                ("Host", "nikiesboldi.ddnsfree.com"),
                ("Accept-Language", "hu"),
            ].iter().cloned().collect(),
            body: "",
            protocol: "HTTP/1.1",
        };
        let users = Arc::new(vec![Mutex::new(User::new("test".to_string(), "test".to_string()))]);
        let mut stream = TcpStream::connect("").unwrap();
        routing(&mut stream, request, users);
    }
}
*/