use crate::*;

#[derive(Debug)]
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