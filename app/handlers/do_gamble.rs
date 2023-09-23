/*!
 * Implementation of "gamble" command.
 */

use crate::handlers::Handler;
use crate::{ActionRow, Button, InteractionRequest, InteractionResponse};

const FREE_AMT: u64 = 5;
const STARTING_AMT: u64 = 0;

fn build_action_row() -> ActionRow {
    let roll_button = Button::new().label("roll").id("roll");

    let free_button = Button::new().label("free").id("free");

    let brag_button = Button::new().label("brag").id("brag");

    let help_button = Button::new().label("help").id("help");

    ActionRow::new()
        .button(roll_button)
        .button(free_button)
        .button(brag_button)
        .button(help_button)
}

fn build_help_message() -> String {
    "## Instructions

Roll to get 0x, 1x, 2x, or 3x odds on your betted points. If you want to end your run, use the \"brag\" button to let others know about your score! 

### Odds:
- 25% 0x
- 25% 1x
- 25% 2x
- 25% 3x
".to_string()
}

fn build_bank(n: u64) -> String {
    "You have: ".to_string() + &n.to_string()
}

pub struct GambleHandler;

impl Handler for GambleHandler {
    fn handle_application_command(&self, _: &InteractionRequest) -> InteractionResponse {
        InteractionResponse::new()
            .message(&build_bank(STARTING_AMT))
            .component_row(build_action_row())
    }

    fn handle_message_component(&self, req: &InteractionRequest) -> InteractionResponse {
        // UNFINISHED BUSINESS HERE
        let amt = 1337;

        match req
            .data
            .as_ref()
            .unwrap()
            .custom_id
            .as_ref()
            .unwrap()
            .as_str()
        {
            "roll" => InteractionResponse::new()
                .message(&build_bank(amt))
                .component_row(build_action_row())
                .edit(),

            "free" => InteractionResponse::new()
                .message(&build_bank(amt + FREE_AMT))
                .component_row(build_action_row())
                .edit(),

            "brag" => InteractionResponse::new()
                .message(&("Winnings: ".to_string() + &amt.to_string()))
                .shout(),

            "help" => InteractionResponse::new()
                .message(&(build_help_message() + "\n" + &build_bank(amt)))
                .component_row(build_action_row())
                .edit(),

            &_ => todo!(),
        }
    }
}
