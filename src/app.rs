use crate::app::interactions::{InteractionRequest, InteractionResponse};

pub mod interactions;

pub fn handle_interaction(request: &InteractionRequest) -> InteractionResponse {
  InteractionResponse{
      r#type : 1
  }
}
