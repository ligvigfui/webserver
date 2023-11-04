use std::net::TcpStream;

use crate::*;

pub fn routing(stream: &mut TcpStream, request: Request, users: Arc<Vec<Mutex<User>>>){
    /*if request.protocol != "HTTP/1.1" {
        println!("Protocol not supported: {}", request.protocol);
        handle_page_return(stream, CODES[&505], None, "505.html");
    }*/
    let request = match request.get_header("Accept-Language") {
        Some(x) => match x.contains("hu") {
            true => request.set_header("Accept-Language", "hu"),
            false => request.set_header("Accept-Language", "en"),
        },
        None => request.set_header("Accept-Language", "en"),
    };

    if DEBUG >= DebugLevel::LOW {
        println!("handeling - {}", request.path);
    }

    match request.get_header("Host").unwrap().split(":").next().unwrap() {
        "nikiesboldi.ddnsfree.com" => wedding::routing(stream, request ),
        "neptuncrf.freeddns.org" => neptunCRF::routing(stream, request, users),
        "coder.ddnsfree.com" => dev::routing(stream, request),
        "localhost" => {
            match request.path.split("/").nth(1).unwrap() {
                "" => handle_page_return(stream, CODES[&200], None, "en/dev.html"),
                "dev" => dev::routing(stream,
                    Request { 
                        path: &request.path.replace("/dev", ""),
                        ..request
                    }),
                "neptunCRF" => neptunCRF::routing(stream,
                    Request { 
                        path: &request.path.replace("/neptunCRF", ""),
                        ..request
                    }, users),
                "wedding" => wedding::routing(stream,
                    Request {
                        path: &request.path.replace("/wedding", ""),
                        ..request
                    }),
                _ => response404(stream, request),
            }
        }
        _ => {
            println!("Did not find host: \"{}\"", request.get_header("Host").unwrap());
            response404(stream, request);
        }
    }
}

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