/*!
 * Implementation of "gamble" command.
 */

use crate::handlers::Handler;
use crate::{ActionRow, Button, InteractionRequest, InteractionResponse};
use regex::Regex;

const FREE_AMT: u64 = 5;
const STARTING_AMT: u64 = 0;
const BANK_PREFIX: &str = "You have: ";
const BANK_SUFFIX: &str = " :tickets:s";

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

Roll to get 0x, 1x, 2x, or 3x odds on your betted :tickets:s. Use the \"brag\" button to tell others how many :tickets:s you have! 

### Odds:
- 25% 0x
- 25% 1x
- 25% 2x
- 25% 3x
".to_string()
}

fn build_bank(n: u64) -> String {
    BANK_PREFIX.to_string() + &n.to_string() + BANK_SUFFIX
}

fn recognize_bank(hay: &str) -> u64 {
    let pattern = BANK_PREFIX.to_string() + "[0-9]*" + BANK_SUFFIX;
    let re = Regex::new(&pattern).unwrap();
    let mut range = re.find(hay).unwrap().range();
    range.start += BANK_PREFIX.len();
    range.end -= BANK_SUFFIX.len();
    hay[range].parse::<u64>().unwrap_or(STARTING_AMT)
}

fn get_user_name(req: &InteractionRequest) -> String {
    match &req.member {
        Some(member) => match &member.user {
            Some(user) => user.id.clone(),
            None => "Someone".to_string(),
        },
        None => "Someone".to_string(),
    }
}

pub struct GambleHandler;

impl Handler for GambleHandler {
    fn handle_application_command(&self, _: &InteractionRequest) -> InteractionResponse {
        InteractionResponse::new()
            .message(&build_bank(STARTING_AMT))
            .component_row(build_action_row())
    }

    fn handle_message_component(&self, req: &InteractionRequest) -> InteractionResponse {
        let amt = recognize_bank(&req.message.as_ref().unwrap().content);

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

            "brag" => {
                let name = get_user_name(req);

                let msg = format!("<@{}> has {} :tickets:s!", name, amt);

                InteractionResponse::new().message(&msg).shout()
            }

            "help" => InteractionResponse::new()
                .message(&(build_help_message() + "\n" + &build_bank(amt)))
                .component_row(build_action_row())
                .edit(),

            &_ => todo!(),
        }
    }
}
