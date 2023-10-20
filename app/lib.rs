/*!
 * Drives the main application logic. Takes in a Discord interaction request, and returns a
 * response according to application rules.
 */

mod handlers;

use discord_interaction::{InteractionType::*, *};
use handlers::{DeedeeHandler, ErrorHandler, GameOfLifeHandler, Handler, ShellsHandler};

pub fn handle_interaction(request: &Request) -> Response {
    match request.r#type {
        Ping => handle_ping(request),

        ApplicationCommand => handle_application_command(request),

        MessageComponent => handle_message_component(request),

        ModalSubmit => handle_modal_submit(request),
    }
}

fn handle_ping(_: &Request) -> Response {
    Response::pong()
}

fn select_handler(name: &str) -> Box<dyn Handler> {
    match name {
        "conway" => Box::new(GameOfLifeHandler),

        "deedee" => Box::new(DeedeeHandler),

        "shells" => Box::new(ShellsHandler),

        _ => Box::new(ErrorHandler),
    }
}

fn handle_application_command(request: &Request) -> Response {
    match request.command_name() {
        Some(name) => select_handler(&name).handle_application_command(request),
        None => make_error_response(),
    }
}

fn handle_message_component(request: &Request) -> Response {
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

fn handle_modal_submit(request: &Request) -> Response {
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

fn make_error_response() -> Response {
    Response::message()
        .content("Something erroneous happened...")
        .into()
}

#[cfg(test)]
mod tests {

    use super::*;
    use handlers::SIZE;

    #[test]
    fn test_ping_pong() {
        let req = Request::ping();

        let resp = handle_interaction(&req);

        assert_eq!(resp, Response::pong());
    }

    #[test]
    fn test_conway() {
        let req: Request = Request::application_command("conway").into();

        let resp = handle_interaction(&req);

        let content = resp.message_content().unwrap();

        let resp_emoji_count = content.matches("🌝").count() + content.matches("🌚").count();

        let expected_emoji_count = SIZE.pow(2) * 2;

        println!("{}", content);
        assert_eq!(expected_emoji_count, resp_emoji_count);
    }

    #[test]
    fn test_deedee() {
        let req = Request::application_command("deedee").into();

        let resp = handle_interaction(&req);

        let expected_resp = Response::message().content("mega doo doo").into();

        assert_eq!(resp, expected_resp);
    }

    #[test]
    fn shell_game() {
        let req = Request::application_command("shells").into();

        let resp = handle_interaction(&req);

        let components = resp.message_components();

        assert_eq!(components.len(), 5);
    }
}
