/*!
 * Drives the main application logic. Takes in a Discord interaction request, and returns a
 * response according to application rules.
 */

mod handlers;
pub mod interactions;

use crate::interactions::InteractionCallbackType::*;
use crate::interactions::InteractionType::*;
use crate::interactions::*;
use handlers::deedee;
use handlers::hello;

pub fn handle_interaction(request: &InteractionRequest) -> InteractionResponse {
    match request.r#type {
        Ping => handle_ping(request),

        ApplicationCommand => handle_application_command(request),
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
        Some(interaction_data) => match interaction_data.name.as_str() {
            "deedee" => deedee(&interaction_data),

            "hello" => hello(&request.member),

            _ => make_error_callback_data(),
        },

        None => make_error_callback_data(),
    };

    InteractionResponse {
        r#type: ChannelMessageWithSource,
        data: Some(callback_data),
    }
}

fn make_error_callback_data() -> InteractionCallbackData {
    InteractionCallbackData {
        content: Some("Could not recognize command.".to_string()),
    }
}

#[cfg(test)]
mod tests {

    use super::*;

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
    fn test_deedee() {
        let req_data = InteractionData {
            name: "deedee".to_string(),
        };

        let req = anonymous_request(ApplicationCommand, Some(req_data));

        let resp = handle_interaction(&req);

        let expected_resp_data = InteractionCallbackData {
            content: Some("mega doo doo".to_string()),
        };

        let expected_resp = InteractionResponse {
            r#type: ChannelMessageWithSource,
            data: Some(expected_resp_data),
        };

        assert_eq!(resp, expected_resp);
    }

    #[test]
    fn test_hello() {
        let member_guildless = None;

        assert_eq!(
            hello(&member_guildless),
            InteractionCallbackData {
                content: Some("Hello, stranger!".to_string()),
            }
        );

        let member_nickless = Some(GuildMember {
            user: None,
            nick: None,
        });

        assert_eq!(
            hello(&member_nickless),
            InteractionCallbackData {
                content: Some("Hello, friend!".to_string()),
            }
        );

        let member_with_nickname = Some(GuildMember {
            user: None,
            nick: Some("Nicky".to_string()),
        });

        assert_eq!(
            hello(&member_with_nickname),
            InteractionCallbackData {
                content: Some("Hello, Nicky!".to_string()),
            }
        );
    }
}
