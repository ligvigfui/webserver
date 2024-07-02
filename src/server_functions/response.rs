use crate::*;

#[derive(Debug)]
pub struct Response {
    pub http_verison: HTTPVerion,
    pub status: StatusCode,
    pub headers: HashMap<Header, String>,
    pub payload: ResponsePayload,
}

impl Default for Response {
    fn default() -> Self {
        Self { http_verison: HTTPVerion::_11, status: StatusCode::_200, headers: HashMap::new(), payload: ResponsePayload::None }
    }
}

impl Response {
    pub fn new(payload: ResponsePayload) -> Self {
        Self {
            http_verison: HTTPVerion::_11,
            status: StatusCode::_200,
            headers: HashMap::new(),
            payload
        }
    }

    pub fn _404(request: &Request) -> Self {
        let host = match request.headers.get(&Host) {
            Some(x) => x,
            None => "noHost",
        };
        let language = match request.headers.get(&AcceptLanguage) {
            Some(x) => x,
            None => "en",
        };
        println!("Error {} - Host: {} requested page: {}", StatusCode::_404, host, &request.path);
        Self {
            http_verison: HTTPVerion::_11,
            status: StatusCode::_404,
            headers: HashMap::new(),
            payload: ResponsePayload::File(PathBuf::from(format!("./pages/{language}/404.html")))
        }
    }

    pub fn as_bytes(&mut self) -> Vec<Vec<u8>> {
        let payload_bites = match self.payload.as_bytes() {
            Ok(x) => x,
            Err(e) => {
                log_error(e);
                return Response::_404(&Request::default()).as_bytes();
            }
        };
        match self.payload.add_headers() {
            Some(payload_headers) => {
                for (k, v) in payload_headers {
                    if !self.headers.contains_key(&k) {
                        self.headers.insert(k, v);
                    }
                };
            },
            None => {},
        };
        self.headers.insert(Header::Server, format!("ligvigfui's rust webserver/{VERSION}"));
        let headers = self.headers.iter()
            .map(|(k, v)| format!("{}: {}", k, v)).collect::<Vec<String>>().join("\r\n");
        
        let result = format!("\
            {} {}\r\n\
            {}\r\n",
            self.http_verison, self.status,
            headers
        );
        self.headers.insert(ContentLength, payload_bites.len().to_string());
        let result_bites = result.as_bytes().to_vec();
        let potential_content_len = format!("{ContentLength}: {}\r\n\r\n", payload_bites.len());
        const TCP_MAX_PACKAT_SIZE: usize = 1400;
        match result_bites.len() + potential_content_len.as_bytes().len() + payload_bites.len() <= TCP_MAX_PACKAT_SIZE {
            true => {
                let mut result = result_bites;
                result.extend_from_slice(potential_content_len.as_bytes());
                result.extend_from_slice(&payload_bites);
                vec![result]
            }
            false => {
                let mut result = result_bites;
                result.extend_from_slice("Transfer-Encoding: chunked\r\n\r\n".as_bytes());
                let mut result = vec![result];
                for chunk in payload_bites.chunks(TCP_MAX_PACKAT_SIZE) {
                    result.push([
                        format!("{:x}\r\n", chunk.len()).as_bytes(),
                        chunk,
                        b"\r\n"
                    ].concat())
                }
                result.push(b"0\r\n\r\n".to_vec());
                result
            }
        }
    }
}