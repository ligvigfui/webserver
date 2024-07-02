use crate::*;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum Header {
    AcceptLanguage,
    AccessControlAllowOrigin,
    Cookies,
    Connection,
    ContentLength,
    ContentType,
    Host,
    Location,
    TransferEncoding,
    Server,
    Some(String),
}

impl From<&str> for Header {
    fn from(str: &str) -> Header {
        match str {
            "Cookies" => Cookies,
            "Content-Type" => ContentType,
            "Access-Control-Allow-Origin" => Header::AccessControlAllowOrigin,
            "Accept-Language" => AcceptLanguage,
            "Connection" => Header::Connection,
            "Content-Length" => ContentLength,
            "Host" => Host,
            "Location" => Header::Location,
            "Transfer-Encoding" => Header::TransferEncoding,
            "Server" => Header::Server,
            some => Header::Some(some.to_string())
        }
    }
}

impl Display for Header {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            AcceptLanguage => "Accept-Language",
            Header::AccessControlAllowOrigin => "Access-Control-Allow-Origin",
            Cookies => "Cookies",
            Header::Connection => "Connection",
            ContentLength => "Content-Length",
            ContentType => "Content-Type",
            Host => "Host",
            Header::Location => "Location",
            Header::TransferEncoding => "Transfer-Encoding",
            Header::Server => "Server",
            Header::Some(h) => h,
        };
        write!(f, "{}", string)
    }
}