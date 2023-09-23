mod do_deedee;
mod do_gamble;
mod do_game_of_life;

pub use do_deedee::DeedeeHandler;
pub use do_gamble::gamble;
pub use do_game_of_life::{game_of_life, SIZE};

use crate::{InteractionCallbackData, InteractionData};

pub trait Handler {
    fn handle_application_command(data: &InteractionData) -> InteractionCallbackData;

    fn handle_message_component(data: &InteractionData) -> InteractionCallbackData {
        Self::handle_application_command(data)
    }
}
