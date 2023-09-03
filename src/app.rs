use crate::app::interactions::*;
use crate::app::InteractionType::*;
use crate::app::InteractionCallbackType::*;

pub mod interactions;

pub fn handle_interaction(request: &InteractionRequest) -> InteractionResponse {

    match request.r#type {

        Ping => InteractionResponse{
            r#type: Pong,
            data: None
        },

        _ => InteractionResponse{
            r#type: ChannelMessageWithSource,
            data: Some(
                InteractionCallbackData {
                    content: Some("response text".to_owned())
                }
            )
        }
    }
}
