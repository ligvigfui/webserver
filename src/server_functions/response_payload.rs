use crate::*;

#[derive(Debug)]
pub enum ResponsePayload {
    None,
    File(PathBuf),
    Json(String),
    Bites(Vec<u8>),
}