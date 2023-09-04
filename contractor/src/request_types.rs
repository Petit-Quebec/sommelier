use serde::Serialize;

#[derive(Serialize)]
pub struct RegistrationRequest {
    pub name: String,
    pub r#type: u32,
    pub description: String,
}
