/*!
 * Drives the main application logic. Takes in a Discord interaction request, and returns a
 * response according to application rules.
 */

mod handlers;
pub mod interactions;

use crate::interactions::InteractionType::*;
use crate::interactions::*;
use handlers::{DeedeeHandler, ErrorHandler, GameOfLifeHandler, Handler, ShellsHandler};

pub fn handle_interaction(request: &InteractionRequest) -> InteractionResponse {
    match request.r#type {
        Ping => handle_ping(request),

        ApplicationCommand => handle_application_command(request),

        MessageComponent => handle_message_component(request),

        ModalSubmit => handle_modal_submit(request),
    }
}

fn handle_ping(_: &InteractionRequest) -> InteractionResponse {
    InteractionResponse::pong()
}

fn select_handler(name: &str) -> Box<dyn Handler> {
    match name {
        "conway" => Box::new(GameOfLifeHandler),

        "deedee" => Box::new(DeedeeHandler),

        "shells" => Box::new(ShellsHandler),

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

fn handle_modal_submit(request: &InteractionRequest) -> InteractionResponse {
    let name = &request
        .message
        .as_ref()
        .unwrap()
        .interaction
        .as_ref()
        .unwrap()
        .name;

    select_handler(name).handle_modal_submit(request)
}

fn make_error_response() -> InteractionResponse {
    InteractionResponse::message()
        .content("Something erroneous happened...")
        .into()
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
            r#type: r#type,
            data: data,
            member: Some(GuildMember {
                user: User {
                    id: "DEBUG_USER_ID".to_string(),
                },
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

        assert_eq!(resp, InteractionResponse::pong());
    }

    #[test]
    fn test_conway() {
        let req_data = InteractionData {
            name: Some("conway".to_string()),
            custom_id: None,
        };

        let req = anonymous_request(ApplicationCommand, Some(req_data));

        let resp = handle_interaction(&req);

        let content = resp.message_content().unwrap();

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

        let expected_resp = InteractionResponse::message()
            .content("mega doo doo")
            .into();

        assert_eq!(resp, expected_resp);
    }

    #[test]
    fn shell_game() {
        let req_data = InteractionData {
            name: Some("shells".to_string()),
            custom_id: None,
        };

        let req = anonymous_request(ApplicationCommand, Some(req_data));

        let resp = handle_interaction(&req);

        let components = resp.message_components();

        assert_eq!(components.len(), 5);
    }

    #[test]
    fn test_shells_free() {
        let req_data = InteractionData {
            name: None,
            custom_id: Some("free".to_string()),
        };

        let mut req = anonymous_request(MessageComponent, Some(req_data));

        let interaction = MessageInteraction {
            name: "shells".to_string(),
        };

        let message = Message {
            content: "You have: 3043 :shell:s".to_string(),
            interaction: Some(interaction),
        };

        req.message = Some(message);

        let resp = handle_interaction(&req);

        assert_eq!(
            resp.message_content().unwrap(),
            "# :woman_elf::magic_wand:\nYou are given 5 free :shell:s.\n*Come again anytime!*\n# Your Stats\nYou have: 3048 :shell:s\nYou have: infinite :zap:".to_string()
        );
    }
}
