use crate::app::interactions::{InteractionRequest, InteractionResponse, InteractionType, InteractionCallbackType};

pub mod interactions;

pub fn handle_interaction(request: &InteractionRequest) -> InteractionResponse {
    match request.r#type {
        _ => InteractionResponse{ r#type: InteractionCallbackType::Pong }
    }
}
