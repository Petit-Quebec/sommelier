mod deedee;
mod error;
mod game_of_life;
mod shells;

pub use deedee::DeedeeHandler;
pub use error::ErrorHandler;
pub use game_of_life::{GameOfLifeHandler, SIZE};
pub use shells::ShellsHandler;

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
