use crate::*;

#[derive(serde::Deserialize, Default, Debug)]
pub struct Form<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub guests: u8,
}