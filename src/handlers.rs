mod do_deedee;
mod do_error;
mod do_game_of_life;
mod do_shells;

pub use do_deedee::DeedeeHandler;
pub use do_error::ErrorHandler;
pub use do_game_of_life::{GameOfLifeHandler, SIZE};
pub use do_shells::ShellsHandler;

use discord_interaction::{Request, Response};

pub trait Handler {
    fn handle_application_command(&self, data: &Request) -> Response;

    fn handle_message_component(&self, data: &Request) -> Response {
        Self::handle_application_command(self, data)
    }

    fn handle_modal_submit(&self, data: &Request) -> Response {
        Self::handle_application_command(self, data)
    }
}
