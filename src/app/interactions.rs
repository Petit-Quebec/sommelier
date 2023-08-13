use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct InteractionRequest {
    id: String,
    application_id: String,
    r#type: u32
}

#[derive(Serialize)]
pub struct InteractionResponse {
    pub r#type: u32
}

