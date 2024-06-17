use crate::*;

#[derive(Debug)]
pub enum ResponsePayload {
    None,
    File(Path),
    Json(String),
    Bites(Vec<u8>),
}