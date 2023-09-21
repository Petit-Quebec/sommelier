/*!
 * Drives the main application logic. Takes in a Discord interaction request, and returns a
 * response according to application rules.
 */

mod handlers;
pub mod interactions;

use crate::interactions::InteractionCallbackType::*;
use crate::interactions::InteractionType::*;
use crate::interactions::*;
use handlers::buttons;
use handlers::deedee;
use handlers::game_of_life;

pub fn handle_interaction(request: &InteractionRequest) -> InteractionResponse {
    match request.r#type {
        Ping => handle_ping(request),

        ApplicationCommand => handle_application_command(request),

        MessageComponent => handle_message_component(request),
    }
}

fn handle_ping(_: &InteractionRequest) -> InteractionResponse {
    InteractionResponse {
        r#type: Pong,
        data: None,
    }
}

fn handle_application_command(request: &InteractionRequest) -> InteractionResponse {
    let callback_data = match &request.data {
        Some(interaction_data) => match &interaction_data.name {
            Some(name) => match name.as_str() {
            
                "buttons" => buttons(&interaction_data),

                "conway" => game_of_life(&interaction_data),

                "deedee" => deedee(&interaction_data),

                _ => make_error_callback_data(),
            },

            None => make_error_callback_data(),
        },

        None => make_error_callback_data(),
    };

    InteractionResponse {
        r#type: ChannelMessageWithSource,
        data: Some(callback_data),
    }
}


fn handle_message_component(request: &InteractionRequest) -> InteractionResponse {
     let callback_data = match &request.data {
        Some(interaction_data) => match &interaction_data.custom_id {
            Some(id) => match id.as_str() {
            
                "cgol" => game_of_life(&interaction_data),

                "deedee" => deedee(&interaction_data),

                _ => make_error_callback_data(),
            },

            None => make_error_callback_data(),
        },

        None => make_error_callback_data(),
    };

    InteractionResponse {
        r#type: UpdateMessage,
        data: Some(callback_data),
    }
}

fn make_error_callback_data() -> InteractionCallbackData {
    InteractionCallbackData {
        content: Some("Could not recognize command.".to_string()),
        components: Vec::new(),
        flags: Some(MessageFlags::Ephemeral),
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use handlers::SIZE;

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
        }
    }

    #[test]
    fn test_ping_pong() {
        let req = anonymous_request(Ping, None);

        let expected_resp = InteractionResponse {
            r#type: Pong,
            data: None,
        };

        let resp = handle_interaction(&req);

        assert_eq!(resp, expected_resp);
    }

    #[test]
    fn test_buttons() {
        let req_data = InteractionData {
            name: Some("buttons".to_string()),
            custom_id: None,
        };

        let req = anonymous_request(ApplicationCommand, Some(req_data));

        let resp = handle_interaction(&req);

        let components = resp.data.unwrap().components;

        assert_eq!(components.len(), 1);

        let buttons = &components[0].components;

        assert_eq!(buttons.len(), 2);

        assert_eq!(buttons[0].r#type, ComponentType::Button);

        assert_eq!(buttons[1].r#type, ComponentType::Button);
    }

    #[test]
    fn test_conway() {
        let req_data = InteractionData {
            name: Some("conway".to_string()),
            custom_id: None,
        };

        let req = anonymous_request(ApplicationCommand, Some(req_data));

        let resp = handle_interaction(&req);

        let content = resp
            .data
            .expect("no data in response!")
            .content
            .expect("no content in data!");

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
            data: Some(expected_resp_data),
        };

        assert_eq!(resp, expected_resp);
    }
}
