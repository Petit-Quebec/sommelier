/*!
 * Drives the main application logic. Takes in a Discord interaction request, and returns a
 * response according to application rules.
 */

use interactions::InteractionCallbackType::*;
use interactions::InteractionType::*;
use interactions::*;

pub mod interactions;

pub fn handle_interaction(request: &InteractionRequest) -> InteractionResponse {
    match request.r#type {
        Ping => InteractionResponse {
            r#type: Pong,
            data: None,
        },

        ApplicationCommand => {
            let metadata = InteractionMetadata {
                user_id: Some("CHANGE_THIS".to_string()),
                channel_id: Some("CHANGE_THIS".to_string()),
                guild_id: Some("CHANGE_THIS".to_string()),
            };

            let result_data = process_application_command(request.data.as_ref(), &metadata);

            InteractionResponse {
                r#type: ChannelMessageWithSource,
                data: Some(result_data),
            }
        }

        _ => InteractionResponse {
            r#type: ChannelMessageWithSource,
            data: Some(InteractionCallbackData {
                content: Some("response text".to_string()),
            }),
        },
    }
}

fn process_application_command(
    data: Option<&InteractionData>,
    metadata: &InteractionMetadata,
) -> InteractionCallbackData {
    InteractionCallbackData {
        content: Some("DUMMY_APP_COMMAND_RESPONSE".to_string()),
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
            content: Some("DUMMY_APP_COMMAND_RESPONSE".to_string()),
        };

        let expected_resp = InteractionResponse {
            r#type: ChannelMessageWithSource,
            data: Some(expected_resp_data),
        };

        let resp = handle_interaction(&req);

        assert_eq!(resp, expected_resp);
    }
}
