/*!
 * Implementation of "gamble" command.
 */

mod messages;
mod sselvish;
mod state;

use crate::handlers::Handler;
use crate::{Component, InteractionRequest, InteractionResponse};
use rand::{thread_rng, Rng};
use state::InteractionState;
use std::collections;

const FREE_AMT: u64 = 5;

fn build_action_row() -> Vec<Component> {
    let roll_button = Component::button().label("roll").id("roll").into();
    let free_button = Component::button().label("free").id("free").into();
    let brag_button = Component::button().label("brag").id("brag").into();
    let recall_button = Component::button().label("recall").id("recall").into();
    let rules_button = Component::button().label("rules").id("rules").into();

    vec![
        roll_button,
        free_button,
        brag_button,
        recall_button,
        rules_button,
    ]
}

fn build_recall_fields() -> Vec<Component> {
    let claim = Component::text_input().label("claim").id("claim").into();
    let proof = Component::text_input().label("proof").id("proof").into();
    vec![claim, proof]
}

fn build_roll_result(mut state: InteractionState) -> String {
    let bet = state.game_state.bet;
    let bank = state.game_state.bank;

    if bet > bank {
        messages::roll_failure_message(state)
    } else {
        let mut rng = thread_rng();
        let roll: u64 = rng.gen_range(0, 4);
        let winnings = roll * bet;
        state.game_state.bank = bank - bet + winnings;
        messages::roll_success_message(roll, bet, state)
    }
}

fn build_free_result(mut state: InteractionState) -> String {
    state.game_state.bank += FREE_AMT;
    messages::free_message(Some(FREE_AMT), None, state)
}

fn build_brag_result(state: InteractionState) -> String {
    let proof = sselvish::proof(&state.user, &state.game_state.bank.to_string());
    messages::brag_message(&proof, state)
}

fn build_recall_submit_result(
    mut state: InteractionState,
    fields: collections::HashMap<String, String>,
) -> String {
    let user_claim = fields.get("claim").unwrap();
    let user_proof = fields.get("proof").unwrap().trim();
    let expected_proof = sselvish::proof(&state.user, user_claim);
    let user_claim = user_claim.parse::<u64>();

    if user_proof == expected_proof && user_claim.is_ok() {
        state.game_state.bank = user_claim.unwrap();

        messages::recall_success_message(user_proof, state)
    } else {
        messages::recall_failure_message(user_proof, state)
    }
}

fn plain_message(msg: &str) -> InteractionResponse {
    InteractionResponse::message()
        .content(msg)
        .components(build_action_row())
        .into()
}

fn quiet_message(msg: &str) -> InteractionResponse {
    plain_message(msg).edit()
}

fn loud_message(msg: &str) -> InteractionResponse {
    InteractionResponse::message().content(msg).shout().into()
}

fn modal(id: &str, title: &str, comps: Vec<Component>) -> InteractionResponse {
    InteractionResponse::modal()
        .id(id)
        .title(title)
        .components(comps)
        .into()
}

pub struct ShellsHandler;

impl Handler for ShellsHandler {
    fn handle_application_command(&self, req: &InteractionRequest) -> InteractionResponse {
        let state: InteractionState = req.into();

        plain_message(&messages::welcome_message(state))
    }

    fn handle_message_component(&self, req: &InteractionRequest) -> InteractionResponse {
        let state: InteractionState = req.into();

        let id = req.custom_id().unwrap();

        let res: InteractionResponse = match id.as_str() {
            "roll" => quiet_message(&build_roll_result(state)),

            "free" => quiet_message(&build_free_result(state)),

            "brag" => loud_message(&build_brag_result(state)),

            "recall" => modal("submit_recall", "Circle of Recall", build_recall_fields()),

            "rules" => quiet_message(&messages::rules_message(state)),

            &_ => panic!("unknown message command"),
        };

        match id.as_str() {
            "brag" => res,
            "recall" => res,
            _ => res.edit(),
        }
    }

    fn handle_modal_submit(&self, req: &InteractionRequest) -> InteractionResponse {
        let id = req.custom_id().unwrap();

        match id.as_str() {
            "submit_recall" => {
                let state: InteractionState = req.into();

                let content = build_recall_submit_result(state, req.modal_submit_values());

                let resp: InteractionResponse = InteractionResponse::message()
                    .content(&content)
                    .components(build_action_row())
                    .into();

                resp.edit()
            }

            &_ => todo!(),
        }
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

        let content = resp.message_content().unwrap();

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

        let resp = ShellsHandler.handle_message_component(&req);

        assert_eq!(
            resp.message_content().unwrap(),
            "# :beach:\nYou find 5 :shell:s.\n## Your Stats\nYou have: 3048 :shell:s\nYou are betting: 0 :shell:s\nYou have: 0 :star2:s\n".to_string()
        );
    }
}
