use crate::*;

pub fn routing(request: &mut Request, users: Arc<Vec<Mutex<User>>>) -> Response {
    /*if request.protocol != HTTPVerion::_11 {
        println!("Protocol not supported: {}", request.protocol);
        return Response {
            http_verison: HTTPVerion::_11,
            status: StatusCode::_505,
            headers: HashMap::new(),
            payload: ResponsePayload::File(PathBuf::from("./pages/en/505.html")),
        };
    }*/
    match request.headers.get(&AcceptLanguage) {
        Some(x) => match x.contains("hu") {
            true => *request.headers.get_mut(&AcceptLanguage).unwrap() = "hu",
            false => *request.headers.get_mut(&AcceptLanguage).unwrap() = "en",
        },
        None => {
            request.headers.insert(AcceptLanguage, "en");
        },
    };
    if DEBUG >= DebugLevel::LOW {
        color!(chrono::Local::now().format("%Y-%m-%d %H:%M:%S")).foreground(&Color::U8(U8Color::new(11))).println();
        color!("Handling - Request.path: {}, Request.method: {:?}", request.path, request.method).foreground(&Color::U8(U8Color::new(21))).println();
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
    let response = wedding::routing(request);
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
    color!("Done with request: {}", request.path).foreground(&Color::U8(U8Color::new(22))).println();
    response
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