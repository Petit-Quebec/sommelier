/*!
 * Drives the main application logic. Takes in a Discord interaction request, and returns a
 * response according to application rules.
 */

use interactions::InteractionCallbackType::*;
use interactions::InteractionType::*;
use interactions::*;

pub mod interactions;

const UNKNOWN_USER: &str = "unknown user";
const UNKNOWN_CHANNEL: &str = "unknown channel";
const UNKNOWN_GUILD: &str = "unknown guild";

fn generate_error_response() -> InteractionResponse {
    let msg = "Something mysterious happened...".to_string();

    let callback_data = InteractionCallbackData { content: Some(msg) };

    InteractionResponse {
        r#type: ChannelMessageWithSource,
        data: Some(callback_data),
    }
}

pub fn handle_interaction(request: &InteractionRequest) -> InteractionResponse {
    match request.r#type {
        Ping => InteractionResponse {
            r#type: Pong,
            data: None,
        },

        ApplicationCommand => handle_application_command(request),
    }
}

fn handle_ping(request: &InteractionRequest) -> InteractionResponse {
    InteractionResponse {
        r#type: Pong,
        data: None,
    }
}

fn handle_application_command(request: &InteractionRequest) -> InteractionResponse {
    match generate_metadata(request) {
        Some(metadata) => {
            let callback_data = InteractionCallbackData {
                content: Some(format!(
                    "u: {}, c: {}, g: {}",
                    metadata.user_id, metadata.channel_id, metadata.guild_id
                )),
            };

            InteractionResponse {
                r#type: ChannelMessageWithSource,
                data: Some(callback_data),
            }
        }

        None => generate_error_response(),
    }
}

fn generate_metadata(request: &InteractionRequest) -> Option<InteractionMetadata> {
    let user_id: &String = &request.user.as_ref()?.id;

    let channel_id: &String = request.channel_id.as_ref()?;

    let guild_id: &String = request.guild_id.as_ref()?;

    let metadata = InteractionMetadata {
        user_id: user_id,
        channel_id: channel_id,
        guild_id: guild_id,
    };

    Some(metadata)
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
            user: Some(User {
                id: "DEBUG_USER_ID".to_string(),
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
    fn test_dig() {
        let req_data = InteractionData {
            name: "dig".to_string(),
        };

        let req = anonymous_request(ApplicationCommand, Some(req_data));

        let expected_resp_data = InteractionCallbackData {
            content: Some("u: DEBUG_USER_ID, c: DEBUG_CHANNEL_ID, g: DEBUG_GUILD_ID".to_string()),
        };

        let expected_resp = InteractionResponse {
            r#type: ChannelMessageWithSource,
            data: Some(expected_resp_data),
        };

        let resp = handle_interaction(&req);

        assert_eq!(resp, expected_resp);
    }
}
