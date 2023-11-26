use crate::*;

#[derive(Debug, Clone)]
pub struct Request<'a> {
    pub method: Method,
    pub path: &'a str,
    pub query: Option<&'a str>,
    pub protocol: &'a str,
    pub headers: HashMap<&'a str, &'a str>,
    pub body: &'a str,
}

impl<'a> Request<'a> {
    pub fn from(buffer: &'a [u8]) -> Option<Request<'a>> {
        let str_buff = match std::str::from_utf8(&buffer) {
            Ok(x) => x,
            Err(e) => {
                println!("{}", e);
                return None;
            },
        };
        let (start_line, headers_and_body) = str_buff.split_once("\r\n")?;
        let mut start_line_cut = start_line.split(" ");

        let (method, path_and_query, protocol) = 
            (start_line_cut.next()?, start_line_cut.next()?, start_line_cut.next()?);

        let (path, query) = match path_and_query.split_once("?") {
            Some((path, query)) => (path, Some(query)),
            None => (path_and_query, None)
        };
        let (headers, body) = headers_and_body.split_once("\r\n\r\n")?;
        
        let headers_iter = headers.split("\r\n");
        let mut headers = HashMap::new();
        for header in headers_iter {
            let mut header_cut = header.split(": ");
            let (header_name, header_value) = (header_cut.next()?, header_cut.next()?);
            headers.insert(header_name, header_value);
        }
        let method = match Method::from(method) {
            Ok(x) => x,
            Err(e) => {
                println!("{}", e);
                return None;
            },
        };
        Some(Request {
            method,
            path,
            query,
            protocol,
            headers,
            body,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_request_creation(){
        let test = format!("GET / HTTP/1.1\r\n{}{}{}{}{}",
            "Host: localhost:7878\r\n",
            "Connection: keep-alive\r\n",
            "Content-Length: 40\r\n",
            "Accept-Language: en-US,en;q=0.9\r\n\r\n",
            "hjafshfas\r\n\r\ndkgsgoaw sdhf\r\nasdkgfvs ewu");
        let test_request = Request::from(test.as_bytes()).unwrap();
        assert_eq!(test_request.method, Method::GET);
        assert_eq!(test_request.path, "/");
        assert_eq!(test_request.protocol, "HTTP/1.1");
        assert_eq!(test_request.headers.get("Host"), Some(&"localhost:7878"));
        assert_eq!(test_request.headers.get("Connection"), Some(&"keep-alive"));
        assert_eq!(test_request.headers.get("Content-Length"), Some(&"40"));
        assert_eq!(test_request.headers.get("Accept-Language"), Some(&"en-US,en;q=0.9"));
        assert_eq!(test_request.body, "hjafshfas\r\n\r\ndkgsgoaw sdhf\r\nasdkgfvs ewu");
    }
}