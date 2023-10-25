use std::net::TcpStream;

use crate::*;

pub fn routing(stream: &mut TcpStream, request: Request, users: Arc<Vec<Mutex<User>>>){
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
        "neptunCRF.freeddns.org" => neptunCRF::routing(stream, request, users),
        "coder.ddnsfree.com" => dev::routing(stream, request),
        "localhost" => {
            match request.path.split("/").next().unwrap() {
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
        _ => response404(stream, request),
    }
}