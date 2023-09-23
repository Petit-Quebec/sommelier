mod do_buttons;
mod do_deedee;
mod do_gamble;
mod do_game_of_life;

pub use do_buttons::{buttons, buttons_minus_one, buttons_plus_one};
pub use do_deedee::deedee;
pub use do_gamble::gamble;
pub use do_game_of_life::{game_of_life, SIZE};
