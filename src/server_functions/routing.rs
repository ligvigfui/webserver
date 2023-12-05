use crate::*;

pub fn routing(stream: &mut TcpStream, request: &mut Request, users: Arc<Vec<Mutex<User>>>){
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
        println!("\x1b[38;5;21mhandling - {}\x1b[0m", request.path);
    }

    match request.headers.get("Host").unwrap().split(":").next().unwrap() {
        "nikiesboldi.ddnsfree.com" => wedding::routing(stream, request),
        "neptuncrf.freeddns.org" => neptunCRF::routing(stream, request, users),
        "coder.ddnsfree.com" => dev::routing(stream, request),
        "localhost" => {
            match request.path.split("/").nth(1).unwrap() {
                "" => handle_page_return(stream, CODE[&200], None, "en/dev.html"),
                "dev" => {
                    request.path = request.path.replacen("/dev", "", 1);
                    dev::routing(stream, request)
                },
                "neptunCRF" => {
                    request.path = request.path.replacen("/neptunCRF", "", 1);
                    neptunCRF::routing(stream, request, users)
                },
                "wedding" => {
                    request.path = request.path.replacen("/wedding", "", 1);
                    wedding::routing(stream, request)
                },
                _ => response404(stream, request),
            }
        }
        _ => {
            println!("Did not find host: \"{:?}\"", request.headers.get("Host"));
            response404(stream, request);
        }
    }
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