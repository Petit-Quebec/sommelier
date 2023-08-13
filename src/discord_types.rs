use serde::Deserialize;

#[derive(Deserialize)]
pub struct DiscordRequest {
    id: String,
    application_id: String,
    r#type: u32
}
