use crate::*;

#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}

impl TryFrom<&str> for Method {
    type Error = &'static str;

    fn try_from(method: &str) -> Result<Self, Self::Error> {
        match method {
            "GET" => Ok(Method::GET),
            "HEAD" => Ok(Method::HEAD),
            "POST" => Ok(Method::POST),
            "PUT" => Ok(Method::PUT),
            "DELETE" => Ok(Method::DELETE),
            "CONNECT" => Ok(Method::CONNECT),
            "OPTIONS" => Ok(Method::OPTIONS),
            "TRACE" => Ok(Method::TRACE),
            "PATCH" => Ok(Method::PATCH),
            _ => Err("Invalid method"),
        }
    }
}