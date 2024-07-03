use crate::*;

#[derive(Debug)]
pub enum Payload {
    None,
    Redirect(String),
    File(PathBuf),
    Json(String),
    Bites(Vec<u8>),
    Text(String),
}

impl Payload {
    fn from(content_type: &str, body: &str) -> Self {
        match content_type {
            "application/json" => Payload::Json(body.to_string()),
            "plain/text" => Payload::Text(body.to_string()),
            _ => Payload::Bites(body.as_bytes().to_vec()),
            
        }
    }
}

impl Payload {
    pub fn as_bytes(&mut self) -> Result<Vec<u8>, io::Error> {
        match self {
            Payload::None => Ok(Vec::new()),
            Payload::File(f) => {
                let mut file = File::open(f)?;
                let mut payload_bites = Vec::new();
                file.read_to_end(&mut payload_bites)?;
                Ok(payload_bites)
            },
            Payload::Json(j) => Ok(j.as_bytes().to_vec()),
            Payload::Bites(b) => Ok(b.to_vec()),
            Payload::Text(t) => Ok(t.as_bytes().to_vec()),
            Payload::Redirect(_) => Ok(Vec::new()),
        }
    }

    pub fn add_headers(&self) -> Option<HashMap<Header, String>> {
        match self {
            Payload::File(f) => {
                let extension = match f.extension() {
                    Some(x) => x.to_str().unwrap(),
                    None => "",
                };
                let mime = match extension {
                    "css" => "text/css",
                    "exe" => "application/vnd.microsoft.portable-executable",
                    "gif" => "image/gif",
                    "html" => "text/html",
                    "jpeg" | "jpg" => "image/jpeg",
                    "js" => "application/javascript",
                    "json" => "application/json",
                    "png" => "image/png",
                    "svg" => "image/svg+xml",
                    "webp" => "image/webp",
                    "" => "application/x-elf",
                    _ => {
                        log_error(format!("Unknown file extension: {:?}", extension));
                        "application/octet-stream"
                    },
                };
                Some(HashMap::from([
                    (Header::ContentType, mime.to_string()),
                ]))
            },
            Payload::Json(_) => Some(HashMap::from([
                (Header::ContentType, "application/json".to_string()),
            ])),
            Payload::Text(_) => Some(HashMap::from([
                (Header::ContentType, "text/plain".to_string()),
            ])),
            Payload::Redirect(p) => Some(HashMap::from([
                (Header::Location, p.to_string()),
            ])),
            _ => None,
        }
    }
}