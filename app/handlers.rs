mod do_deedee;
mod do_error;
mod do_game_of_life;
mod do_shells;

pub use do_deedee::DeedeeHandler;
pub use do_error::ErrorHandler;
pub use do_game_of_life::{GameOfLifeHandler, SIZE};
pub use do_shells::{recognize_bank, ShellsHandler};

use crate::{InteractionRequest, InteractionResponse};

pub trait Handler {
    fn handle_application_command(&self, data: &InteractionRequest) -> InteractionResponse;

    fn handle_message_component(&self, data: &InteractionRequest) -> InteractionResponse {
        Self::handle_application_command(self, data)
    }
}
