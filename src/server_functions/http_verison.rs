use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HTTPVerion {
    _11,
    _2,
    _3
}

impl Display for HTTPVerion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HTTPVerion::_11 => write!(f, "HTTP/1.1"),
            HTTPVerion::_2 =>  write!(f, "HTTP/2"),
            HTTPVerion::_3 =>  write!(f, "HTTP/3"),
        }
    }
}

impl TryFrom<&str> for HTTPVerion {
    type Error =  &'static str;

    fn try_from(string: &str) -> Result<Self, Self::Error> {
        match string {
            "HTTP/1.1" => Ok(HTTPVerion::_11),
            "HTTP/2" => Ok(HTTPVerion::_2),
            "HTTP/3" => Ok(HTTPVerion::_3),
            _ => Err("Invalid HTTP Version"),
        }
    }
}