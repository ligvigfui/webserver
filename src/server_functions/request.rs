use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request<'a> {
    pub method: Method,
    pub path: String,
    pub query: Option<&'a str>,
    pub protocol: HTTPVerion,
    pub headers: HashMap<Header, &'a str>,
    pub cookies: Option<HashMap<&'a str, &'a str>>,
    pub body: &'a str,
}

impl Default for Request<'_> {
    fn default() -> Self {
        Self {
            method: Method::GET,
            path: "/".to_string(),
            query: None,
            protocol: HTTPVerion::_11,
            headers: HashMap::new(),
            cookies: None,
            body: "",
        }
    }
}

impl<'a> TryFrom<&'a String> for Request<'a> {
    type Error = &'a str;

    fn try_from(buffer: &'a String) -> Result<Self, Self::Error> {
        let (start_line, headers_and_body) = buffer.split_once("\r\n").err_if_none()?;
        let mut start_line_cut = start_line.split(" ");

        let method = start_line_cut.next().err_if_none()?;
        let path_and_query = start_line_cut.next().err_if_none()?;
        let protocol = HTTPVerion::try_from(start_line_cut.next().err_if_none()?)?;

        let (path, query) = match path_and_query.split_once("?") {
            Some((path, query)) => (path.to_string(), Some(query)),
            None => (path_and_query.to_string(), None)
        };
        let (headers, body) = headers_and_body.split_once("\r\n\r\n").err_if_none()?;
        
        let headers_iter = headers.split("\r\n");
        let mut headers = HashMap::new();
        for header in headers_iter {
            let header_cut = header.split_once(":").err_if_none()?;
            let (header_name, header_value) = (Header::from(header_cut.0), header_cut.1.trim());
            headers.insert(header_name, header_value);
        }
        let cookies = match headers.get(&Header::Cookies) {
            Some(cookies) => {
                let mut hash_map = HashMap::new();
                for cookie in cookies.split(';') {
                    let cookie_cut = cookie.split_once('=').unwrap();
                    hash_map.insert(cookie_cut.0, cookie_cut.1.trim());
                }
                Some(hash_map)
            }
            None => None,
        };
        Ok(Request
        {
            method: Method::try_from(method)?,
            path,
            query,
            protocol,
            headers,
            cookies,
            body,
        })
    }    
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_request_creation(){
        let test = format!("GET / HTTP/1.1\r\n\
            Host: localhost:7878\r\n\
            Connection: keep-alive\r\n\
            Content-Length: 40\r\n\
            Accept-Language:en-US,en;q=0.9\r\n\
            \r\n\
            hjafshfas\r\n\
            \r\n\
            dkgsgoaw sdhf\r\n\
            asdkgfvs ewu");
        let test_request = Request::try_from(&test).unwrap();
        assert_eq!(test_request.method, Method::GET);
        assert_eq!(test_request.path, "/");
        assert_eq!(test_request.protocol, HTTPVerion::_11);
        assert_eq!(test_request.headers.get(&Host), Some(&"localhost:7878"));
        assert_eq!(test_request.headers.get(&Connection), Some(&"keep-alive"));
        assert_eq!(test_request.headers.get(&ContentLength), Some(&"40"));
        assert_eq!(test_request.headers.get(&AcceptLanguage), Some(&"en-US,en;q=0.9"));
        assert_eq!(test_request.body, "hjafshfas\r\n\r\ndkgsgoaw sdhf\r\nasdkgfvs ewu");
    }
}