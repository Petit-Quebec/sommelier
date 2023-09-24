/*!
 * Drives the main application logic. Takes in a Discord interaction request, and returns a
 * response according to application rules.
 */

mod handlers;
pub mod interactions;

use crate::interactions::InteractionType::*;
use crate::interactions::*;
use handlers::{DeedeeHandler, ErrorHandler, GambleHandler, GameOfLifeHandler, Handler};

pub fn handle_interaction(request: &InteractionRequest) -> InteractionResponse {
    match request.r#type {
        Ping => handle_ping(request),

        ApplicationCommand => handle_application_command(request),

        MessageComponent => handle_message_component(request),
    }
}

fn handle_ping(_: &InteractionRequest) -> InteractionResponse {
    InteractionResponse::pong()
}

fn select_handler(name: &str) -> Box<dyn Handler> {
    match name {
        "conway" => Box::new(GameOfLifeHandler),

        "deedee" => Box::new(DeedeeHandler),

        "gamble" => Box::new(GambleHandler),

        _ => Box::new(ErrorHandler),
    }
}

fn handle_application_command(request: &InteractionRequest) -> InteractionResponse {
    match &request.data {
        Some(interaction_data) => match &interaction_data.name {
            Some(name) => select_handler(name).handle_application_command(request),

            None => make_error_response(),
        },

        None => make_error_response(),
    }
}

fn handle_message_component(request: &InteractionRequest) -> InteractionResponse {
    let name = &request
        .message
        .as_ref()
        .unwrap()
        .interaction
        .as_ref()
        .unwrap()
        .name;

    select_handler(name).handle_message_component(request)
}

fn make_error_response() -> InteractionResponse {
    InteractionResponse::new().message("Something erroneous happened...")
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::InteractionCallbackType::*;
    use handlers::{recognize_bank, SIZE};

    fn anonymous_request(
        r#type: InteractionType,
        data: Option<InteractionData>,
    ) -> InteractionRequest {
        InteractionRequest {
            id: "DEBUG_INTERACTION_ID".to_string(),
            application_id: "DEBUG_APP_ID".to_string(),
            r#type: r#type,
            data: data,
            guild_id: Some("DEBUG_GUILD_ID".to_string()),
            channel_id: Some("DEBUG_CHANNEL_ID".to_string()),
            member: Some(GuildMember {
                user: Some(User {
                    id: "DEBUG_USER_ID".to_string(),
                }),
                nick: Some("DEBUG_NICKNAME".to_string()),
            }),
            message: Some(Message {
                content: "DEBUG_MESSAGE_CONTENT".to_string(),
                interaction: None,
            }),
        }
    }

    #[test]
    fn test_ping_pong() {
        let req = anonymous_request(Ping, None);

        let resp = handle_interaction(&req);

        assert_eq!(resp.r#type, InteractionCallbackType::Pong);
    }

    #[test]
    fn test_conway() {
        let req_data = InteractionData {
            name: Some("conway".to_string()),
            custom_id: None,
        };

        let req = anonymous_request(ApplicationCommand, Some(req_data));

        let resp = handle_interaction(&req);

        let content = resp.data.content.expect("no content in data!");

        let resp_emoji_count = content.matches("🌝").count() + content.matches("🌚").count();

        let expected_emoji_count = SIZE.pow(2) * 2;

        println!("{}", content);
        assert_eq!(expected_emoji_count, resp_emoji_count);
    }

    #[test]
    fn test_deedee() {
        let req_data = InteractionData {
            name: Some("deedee".to_string()),
            custom_id: None,
        };

        let req = anonymous_request(ApplicationCommand, Some(req_data));

        let resp = handle_interaction(&req);

        let expected_resp_data = InteractionCallbackData {
            content: Some("mega doo doo".to_string()),
            components: Vec::new(),
            flags: Some(MessageFlags::Ephemeral),
        };

        let expected_resp = InteractionResponse {
            r#type: ChannelMessageWithSource,
            data: expected_resp_data,
        };

        assert_eq!(resp, expected_resp);
    }

    #[test]
    fn test_gamble() {
        let req_data = InteractionData {
            name: Some("gamble".to_string()),
            custom_id: None,
        };

        let req = anonymous_request(ApplicationCommand, Some(req_data));

        let resp = handle_interaction(&req);

        let components = resp.data.components;

        assert_eq!(components.len(), 1);

        let buttons = &components[0].components;

        assert_eq!(buttons.len(), 4);

        assert_eq!(buttons[0].r#type, ComponentType::Button);
        assert_eq!(buttons[1].r#type, ComponentType::Button);
        assert_eq!(buttons[2].r#type, ComponentType::Button);
        assert_eq!(buttons[3].r#type, ComponentType::Button);
    }

    #[test]
    fn test_gamble_roll() {
        let req_data = InteractionData {
            name: None,
            custom_id: Some("roll".to_string()),
        };

        let mut req = anonymous_request(MessageComponent, Some(req_data));

        let interaction = MessageInteraction {
            name: "gamble".to_string(),
        };

        let message = Message {
            content: "You have: 3043 :tickets:s".to_string(),
            interaction: Some(interaction),
        };

        req.message = Some(message);

        let resp = handle_interaction(&req);

        let new_bank = recognize_bank(&resp.data.content.unwrap());

        assert_eq!(new_bank % 3043, 0);
    }

    #[test]
    fn test_gamble_free() {
        let req_data = InteractionData {
            name: None,
            custom_id: Some("free".to_string()),
        };

        let mut req = anonymous_request(MessageComponent, Some(req_data));

        let interaction = MessageInteraction {
            name: "gamble".to_string(),
        };

        let message = Message {
            content: "You have: 3043 :tickets:s".to_string(),
            interaction: Some(interaction),
        };

        req.message = Some(message);

        let resp = handle_interaction(&req);

        assert_eq!(
            resp.data.content.unwrap(),
            "You have: 3048 :tickets:s".to_string()
        );
    }
}
