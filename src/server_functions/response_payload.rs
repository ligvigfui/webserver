use crate::*;

#[derive(Debug)]
pub enum ResponsePayload {
    None,
    Redirect(String),
    File(PathBuf),
    Json(String),
    Bites(Vec<u8>),
}
