use crate::*;

#[derive(Debug)]
pub enum ResponsePayload {
    None,
    Redirect(String),
    File(PathBuf),
    Json(String),
    Bites(Vec<u8>),
}

impl ResponsePayload {
    pub fn as_bytes(&mut self) -> Result<Vec<u8>, io::Error> {
        match self {
            ResponsePayload::None => Ok(Vec::new()),
            ResponsePayload::File(f) => {
                let mut file = File::open(f)?;
                let mut payload_bites = Vec::new();
                file.read_to_end(&mut payload_bites)?;
                Ok(payload_bites)
            },
            ResponsePayload::Json(j) => Ok(j.as_bytes().to_vec()),
            ResponsePayload::Bites(b) => Ok(b.to_vec()),
            ResponsePayload::Redirect(_) => Ok(Vec::new()),
        }
    }

    pub fn add_headers(&self) -> Option<HashMap<Header, String>> {
        match self {
            ResponsePayload::File(f) => {
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
            ResponsePayload::Json(_) => Some(HashMap::from([
                (Header::ContentType, "application/json".to_string()),
            ])),
            ResponsePayload::Redirect(p) => Some(HashMap::from([
                (Header::Location, p.to_string()),
            ])),
            _ => None,
        }
    }
}