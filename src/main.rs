/*!
 * Drives the main application logic. Takes in a Discord interaction request, and returns a
 * response according to application rules.
 */

mod deedee;
mod error;
mod game_of_life;
mod shells;

use deedee::DeedeeHandler;
use discord_interaction::{
    run, ApplicationCommand, InteractionHandler, Message, MessageComponent, Modal, ModalSubmit,
    Response,
};
use error::ErrorHandler;
use game_of_life::GameOfLifeHandler;
use lambda_http::Error;
use shells::ShellsHandler;

const APPLICATION_PUBLIC_KEY: Option<&'static str> = option_env!("SOMMELIER_PUBLIC_KEY");

#[tokio::main]
async fn main() -> Result<(), Error> {
    run::<Sommelier>(APPLICATION_PUBLIC_KEY.unwrap_or("")).await
}

struct Sommelier;

impl InteractionHandler for Sommelier {
    fn handle_application_command(ac: ApplicationCommand) -> Response {
        match ac.command_name.as_str() {
            "conway" => GameOfLifeHandler::handle_application_command(ac),
            "deedee" => DeedeeHandler::handle_application_command(ac),
            "shells" => ShellsHandler::handle_application_command(ac),
            _ => panic!(),
        }
    }

    fn handle_message_component(mc: MessageComponent) -> Response {
        match mc.id.as_str() {
            "conway" => GameOfLifeHandler::handle_message_component(mc),
            "deedee" => DeedeeHandler::handle_message_component(mc),
            "shells" => ShellsHandler::handle_message_component(mc),
            _ => panic!(),
        }
    }

    fn handle_modal_submit(ms: ModalResponse) -> Response {
        match ms.id.as_str() {
            "conway" => GameOfLifeHandler::handle_modal_submit(ac),
            "deedee" => DeedeeHandler::handle_modal_submit(ac),
            "shells" => ShellsHandler::handle_modal_submit(ac),
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use game_of_life::SIZE;

    const INTERACTION_HANDLER: Sommelier = Sommelier {};

    #[test]
    fn test_conway() {
        let req: Request = Request::application_command("conway").into();

        let resp = INTERACTION_HANDLER.handle_interaction(&req);

        let content = resp.message_content().unwrap();

        let resp_emoji_count = content.matches("🌝").count() + content.matches("🌚").count();

        let expected_emoji_count = SIZE.pow(2) * 2;

        println!("{}", content);
        assert_eq!(expected_emoji_count, resp_emoji_count);
    }

    #[test]
    fn test_deedee() {
        let req = Request::application_command("deedee").into();

        let resp = INTERACTION_HANDLER.handle_interaction(&req);

        let expected_resp = Response::message().content("mega doo doo").into();

        assert_eq!(resp, expected_resp);
    }

    #[test]
    fn shell_game() {
        let req = Request::application_command("shells").into();

        let resp = INTERACTION_HANDLER.handle_interaction(&req);

        let components = resp.message_components();

        assert_eq!(components.len(), 5);
    }
}
