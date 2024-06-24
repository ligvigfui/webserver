use crate::*;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Header {
    AcceptLanguage,
    AccessControlAllowOrigin,
    Cookies,
    Connection,
    ContentLength,
    ContentType,
    Host,
    Some(String),
}

impl Header {
    pub fn from(str: &str) -> Self {
        match str {
            "Cookies" => Cookies,
            "Content-Type" => ContentType,
            "Access-Control-Allow-Origin" => Self::AccessControlAllowOrigin,
            "Accept-Language" => AcceptLanguage,
            "Connection" => Header::Connection,
            "Content-Length" => ContentLength,
            "Host" => Host,
            some => Header::Some(some.to_string())
        }
    }
}

impl Display for Header {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            AcceptLanguage => "Accept-Language",
            Self::AccessControlAllowOrigin => "Access-Control-Allow-Origin",
            Cookies => "Cookies",
            Header::Connection => "Connection",
            ContentLength => "Content-Length",
            ContentType => "Content-Type",
            Host => "Host",
            Header::Some(h) => h,
        };
        write!(f, "{}", string)
    }
}