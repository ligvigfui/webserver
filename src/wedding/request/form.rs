use crate::Request;

#[derive(serde::Deserialize, Default, Debug)]
pub struct Form<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub guests: u8,
}

impl<'a> Request<'a> {
    pub fn to_form(&'a self) -> Result<Form<'a>, serde_json::Error> {
        let form: Form = serde_json::from_str(&self.body)?;
        Ok(form)
    }
}