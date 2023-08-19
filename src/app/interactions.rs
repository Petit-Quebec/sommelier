use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct InteractionRequest {
    pub id: String,
    pub application_id: String,
    pub r#type: u32
}

#[derive(Serialize)]
pub struct InteractionResponse {
    pub r#type: u32
}

