/*!
 * Implementation of "gamble" command.
 */

use crate::handlers::Handler;
use crate::{ActionRow, Button, InteractionCallbackData, InteractionData, MessageFlags};

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
    fn handle_application_command(&self, _: &InteractionData) -> InteractionCallbackData {
        InteractionCallbackData {
            content: Some(build_bank(STARTING_AMT)),
            components: vec![build_action_row()],
            flags: Some(MessageFlags::Ephemeral),
        }
    }

    fn handle_message_component(&self, data: &InteractionData) -> InteractionCallbackData {
        // UNFINISHED BUSINESS HERE
        let amt = 1337;

        match data.custom_id.as_ref().unwrap().as_str() {
            "roll" => InteractionCallbackData {
                content: Some(build_bank(amt)),
                components: vec![build_action_row()],
                flags: Some(MessageFlags::Ephemeral),
            },

            "free" => InteractionCallbackData {
                content: Some(build_bank(amt + FREE_AMT)),
                components: vec![build_action_row()],
                flags: Some(MessageFlags::Ephemeral),
            },

            "brag" => InteractionCallbackData {
                content: Some("Winnings: ".to_string() + &amt.to_string()),
                components: vec![],
                flags: None,
            },

            "help" => InteractionCallbackData {
                content: Some(build_help_message() + "\n" + &build_bank(amt)),
                components: vec![build_action_row()],
                flags: Some(MessageFlags::Ephemeral),
            },

            &_ => todo!(),
        }
    }
}
