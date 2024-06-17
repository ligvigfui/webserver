use crate::*;

#[derive(Debug, Clone)]
pub struct Response {
    pub http_verison: HTTPVerion,
    pub status: StatusCode,
    pub headers: Option<HashMap<String, String>>,
    pub payload: ResponsePayload,
}

impl Default for Response {
    fn default() -> Self {
        Self { http_verison: HTTPVerion::_11, status: StatusCode::_200, headers: None, payload: ResponsePayload::None }
    }
}

impl Response {
    pub fn new(payload: ResponsePayload) -> Self {
        Self {
            http_verison: HTTPVerion::_11,
            status: StatusCode::_200,
            headers: None,
            payload
        }
    }

    pub fn _404(request: &Request) -> Self {
        let host = match request.headers.get("Host") {
            Some(x) => x,
            None => "noHost",
        };
        let language = match request.headers.get("Accept-Language") {
            Some(x) => x,
            None => "en",
        };
        println!("Error {} - Host: {} requested page: {}", CODE[&404], host, &request.path);
        Self {
            http_verison: HTTPVerion::_11,
            status: StatusCode::_404,
            headers: None,
            payload: ResponsePayload::File(Path::new(&format!("./pages/{language}/404.html")))
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        
        let response = format!(
            "HTTP/1.1 {status}\r\n\
            {headers_together}\r\n\
            {contents}",
        );
        let mut result = format!("\
            {self.http_verison} {self.status}\r\n\
            {self.headers}",
            
        );
    }
}