use crate::*;

#[derive(Debug)]
pub enum StatusCode {
    _200,
    _400,
    _404,
    _500,
    _505,
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StatusCode::_200 => write!(f, "200 Ok"),
            StatusCode::_400 => write!(f, "400 Bad Request"),
            StatusCode::_404 => write!(f, "404 Not Found"),
            StatusCode::_500 => write!(f, "500 Internal Server Error"),
            StatusCode::_505 => write!(f, "505 HTTP Version Not Supported"),
        }
    }
}