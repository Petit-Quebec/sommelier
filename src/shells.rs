/*!
 * Implementation of "gamble" command.
 */

mod interaction_wrappers;
mod messages;
mod sselvish;
mod state;

use crate::Handler;
use discord_interaction::{Request, Response};
use interaction_wrappers::{edit_message, new_message, recall_modal, set_roll_modal};
use rand::{thread_rng, Rng};
use state::InteractionState;
use std::{cmp, collections};

const FREE_SHELLS_AMT: u64 = 5;
const FREE_INSP_AMT: u64 = 1;

pub struct ShellsHandler;

impl Handler for ShellsHandler {
    fn handle_application_command(&self, req: &Request) -> Response {
        let state: InteractionState = req.into();
        new_message(&messages::welcome_message(&state))
    }

    fn handle_message_component(&self, req: &Request) -> Response {
        let state: InteractionState = req.into();
        let id = req.custom_id().unwrap();

        let res: Response = match id.as_str() {
            "roll" => edit_message(&roll_result(state)),
            "set_roll" => set_roll_modal("set_roll", "Set Roll Amount"),
            "free" => edit_message(&free_result(state)),
            "proof" => edit_message(&proof_result(state)),
            "recall" => recall_modal("submit_recall", "Circle of Recall"),
            &_ => panic!("unknown message command"),
        };

        res
    }

    fn handle_modal_submit(&self, req: &Request) -> Response {
        let state: InteractionState = req.into();
        let values = req.modal_submit_values();
        let id = req.custom_id().unwrap();

        match id.as_str() {
            "submit_recall" => edit_message(&recall_submit_result(state, values)),
            "set_roll" => edit_message(&set_roll_submit_result(state, values)),
            &_ => todo!(),
        }
    }
}

fn roll_result(mut state: InteractionState) -> String {
    let bet = state.game_state.bet;
    let bank = state.game_state.bank;

    if bet > bank {
        messages::roll_failure_message(&state)
    } else {
        let roll: u64 = thread_rng().gen_range(0, 4);
        let winnings = roll * bet;
        state.game_state.bank = bank - bet + winnings;
        state.game_state.bet = cmp::min(state.game_state.bet, state.game_state.bank);
        messages::roll_success_message(bet, roll, &state)
    }
}

fn free_result(mut state: InteractionState) -> String {
    let roll: u8 = thread_rng().gen_range(0, 4);
    match roll {
        // 25% chance
        0 => {
            state.game_state.insp += FREE_INSP_AMT;
            messages::free_message(None, Some(FREE_INSP_AMT), &state)
        }

        // 75% chance
        1.. => {
            state.game_state.bank += FREE_SHELLS_AMT;
            messages::free_message(Some(FREE_SHELLS_AMT), None, &state)
        }
    }
}

fn proof_result(mut state: InteractionState) -> String {
    if state.game_state.insp > 0 {
        state.game_state.insp -= 1;
        let proof = sselvish::proof(&state.user, &state.game_state.bank.to_string());
        messages::proof_success_message(&proof, &state)
    } else {
        messages::proof_failure_message(&state)
    }
}

fn recall_submit_result(
    mut state: InteractionState,
    fields: collections::HashMap<String, String>,
) -> String {
    let user_claim = fields.get("claim").unwrap();
    let user_proof = fields.get("proof").unwrap().trim();
    let expected_proof = sselvish::proof(&state.user, user_claim);
    let user_claim = user_claim.parse::<u64>();

    if user_proof == expected_proof && user_claim.is_ok() {
        state.game_state.bank = user_claim.unwrap();

        messages::recall_success_message(user_proof, &state)
    } else {
        messages::recall_failure_message(user_proof, &state)
    }
}

fn set_roll_submit_result(
    mut state: InteractionState,
    fields: collections::HashMap<String, String>,
) -> String {
    let bet = fields.get("roll_amt").unwrap().parse::<u64>();

    if bet.is_ok() {
        let bet = bet.unwrap();
        if bet <= state.game_state.bank {
            state.game_state.bet = bet;
            messages::set_roll_success_message(bet, &state)
        } else {
            messages::set_roll_amt_failure_message(&state)
        }
    } else {
        messages::set_roll_parse_failure_message(&state)
    }
}

#[cfg(test)]
mod tests {

    use super::state::GameState;
    use super::*;
    use discord_interaction::{GuildMember, Message, MessageInteraction};

    #[test]
    fn roll() {
        let interaction = MessageInteraction {
            name: "shells".to_string(),
        };

        let message = Message {
            content: "You have: 3043 :shell:s".to_string(),
            interaction: Some(interaction),
        };

        let req: Request = Request::message_component("roll", 0).into();

        let req = req.message(message).member(GuildMember::new("some user"));

        let resp = ShellsHandler.handle_message_component(&req);

        let content = &resp.message_content().unwrap();

        let state: GameState = content.into();

        assert_eq!(state.bank % 3043, 0);
    }

    #[test]
    fn free() {
        let interaction = MessageInteraction {
            name: "shells".to_string(),
        };

        let message = Message {
            content: "You have: 3043 :shell:s".to_string(),
            interaction: Some(interaction),
        };

        let req: Request = Request::message_component("free", 0).into();

        let req = req.message(message).member(GuildMember::new("some user"));

        let resp_content = &ShellsHandler
            .handle_message_component(&req)
            .message_content()
            .unwrap();

        let new: GameState = resp_content.into();

        assert!(new.bank == 3048 || new.insp == 1);
        assert!(new.bank == 3043 || new.insp == 0);
    }
}
