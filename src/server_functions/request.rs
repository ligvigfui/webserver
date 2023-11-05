use crate::*;

#[derive(Debug)]
pub struct Request<'a> {
    pub method: Method,
    pub path: &'a str,
    pub protocol: &'a str,
    pub headers: Vec<(&'a str, &'a str)>,
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
        let (method, path, protocol) = 
            (start_line_cut.next()?, start_line_cut.next()?, start_line_cut.next()?);
        let (headers, body) = headers_and_body.split_once("\r\n\r\n")?;
        let headers_iter = headers.split("\r\n");
        let mut headers = Vec::new();
        for header in headers_iter {
            let mut header_cut = header.split(": ");
            let (header_name, header_value) = (header_cut.next()?, header_cut.next()?);
            headers.push((header_name, header_value));
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
            protocol,
            headers,
            body,
        })
    }

    pub fn get_header(&self, header_name: &str) -> Option<&str> {
        for header in &self.headers {
            if header.0 == header_name {
                return Some(header.1);
            }
        }
        None
    }

    /// Sets a header to a new value.
    /// # Returns
    /// A NEW Request with the header set to the new value.
    pub fn set_header(&self, header_name: &'a str, header_value: &'a str) -> Self {
        let mut new_headers = Vec::new();
        for header in &self.headers {
            if header.0 == header_name {
                new_headers.push((header_name, header_value));
            }
            else {
                new_headers.push(*header);
            }
        }
        Request {
            headers: new_headers,
            .. *self
        }
    }
}