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

impl From<u16> for StatusCode {
    fn from(code: u16) -> Self {
        match code {
            200 => StatusCode::_200,
            400 => StatusCode::_400,
            404 => StatusCode::_404,
            500 => StatusCode::_500,
            505 => StatusCode::_505,
            _ => StatusCode::_500,
        }
    }
}