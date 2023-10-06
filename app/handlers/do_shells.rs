/*!
 * Implementation of "gamble" command.
 */

mod interaction_wrappers;
mod messages;
mod sselvish;
mod state;

use crate::handlers::Handler;
use crate::{InteractionRequest, InteractionResponse};
use interaction_wrappers::{
    loud_message, plain_message, quiet_message, recall_modal, set_roll_modal,
};
use rand::{thread_rng, Rng};
use state::InteractionState;
use std::collections;

const FREE_SHELLS_AMT: u64 = 5;
const FREE_INSP_AMT: u64 = 1;

pub struct ShellsHandler;

impl Handler for ShellsHandler {
    fn handle_application_command(&self, req: &InteractionRequest) -> InteractionResponse {
        let state: InteractionState = req.into();
        plain_message(&messages::welcome_message(&state))
    }

    fn handle_message_component(&self, req: &InteractionRequest) -> InteractionResponse {
        let state: InteractionState = req.into();

        let id = req.custom_id().unwrap();

        let res: InteractionResponse = match id.as_str() {
            "roll" => quiet_message(&roll_result(state)),
            "set_roll" => set_roll_modal("set_roll", "Set Roll Amount"),
            "free" => quiet_message(&free_result(state)),
            "brag" => loud_message(&brag_result(state)),
            "recall" => recall_modal("submit_recall", "Circle of Recall"),
            &_ => panic!("unknown message command"),
        };

        res
    }

    fn handle_modal_submit(&self, req: &InteractionRequest) -> InteractionResponse {
        let id = req.custom_id().unwrap();

        match id.as_str() {
            "submit_recall" => {
                let state: InteractionState = req.into();
                let content = recall_submit_result(state, req.modal_submit_values());
                quiet_message(&content)
            }

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
        let mut rng = thread_rng();
        let roll: u64 = rng.gen_range(0, 4);
        let winnings = roll * bet;
        state.game_state.bank = bank - bet + winnings;
        messages::roll_success_message(roll, bet, &state)
    }
}

fn free_result(mut state: InteractionState) -> String {
    let mut rng = thread_rng();
    let roll: u8 = rng.gen_range(0, 255);
    match roll {
        0..=127 => {
            state.game_state.insp += FREE_INSP_AMT;
            messages::free_message(None, Some(FREE_INSP_AMT), &state)
        }
        128.. => {
            state.game_state.bank += FREE_SHELLS_AMT;
            messages::free_message(Some(FREE_SHELLS_AMT), None, &state)
        }
    }
}

fn brag_result(state: InteractionState) -> String {
    let proof = sselvish::proof(&state.user, &state.game_state.bank.to_string());
    messages::brag_message(&proof, &state)
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

#[cfg(test)]
mod tests {

    use super::*;
    use crate::handlers::do_shells::state::GameState;
    use crate::{GuildMember, Message, MessageInteraction};

    #[test]
    fn roll() {
        let interaction = MessageInteraction {
            name: "shells".to_string(),
        };

        let message = Message {
            content: "You have: 3043 :shell:s".to_string(),
            interaction: Some(interaction),
        };

        let req: InteractionRequest = InteractionRequest::message_component("roll", 0).into();

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

        let req: InteractionRequest = InteractionRequest::message_component("free", 0).into();

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
