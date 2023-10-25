use phf::phf_map;

pub static CODES: phf::Map<u16, &'static str> = phf_map! {
    200u16 => "200 OK",
    400u16 => "400 Bad Request",
    404u16 => "404 Not Found",
    500u16 => "500 Internal Server Error",
    501u16 => "501 Not Implemented",
    505u16 => "505 HTTP Version Not Supported",
};