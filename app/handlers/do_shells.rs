/*!
 * Implementation of "gamble" command.
 */

mod state;

use crate::handlers::Handler;
use crate::{Component, InteractionRequest, InteractionResponse};
use hex::FromHex;
use rand::{thread_rng, Rng};
use sha256::digest;
use state::InteractionState;
use std::collections;

const SALT: &str = env!("SOMMELIER_GAMBLING_SALT");
const FREE_AMT: u64 = 5;
const BANK_PREFIX: &str = "You have: ";
const BANK_SUFFIX: &str = " :shell:s";
const INSP_PREFIX: &str = "You have: ";
const INSP_SUFFIX: &str = " :zap:";
const PROOF_LENGTH: usize = 12;

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

fn build_rules_message() -> String {
    "# :woman_elf::shell: Shell Game :shell:

**Roll** to bet your :shell:s, to receive 0x, 1x, 2x, or 3x the amount :shell:s back, with equal probability.

Additionally, whenever you roll, you have a chance to gain :zap:. The more of your saved :shell:s you roll on, the higher the chance that you'll gain :zap:.

**Free** will give you a small number of :shell:s for free! No charge at all.

**Brag** will consume :zap: to **brag** about your score. Let your friends know how many :shell:s you've got! When you brag, you'll also be provided with proof of your achievement in **Sselvish**, a cryptographically secure dialect of Common Elvish. 

**Recall** allows you to reset your current gambling run to a past gambling run that you **bragged** about. So make sure to **brag** often!".to_string()
}

fn build_stats(n: u64) -> String {
    "# Your Stats\n".to_string()
        + BANK_PREFIX
        + &n.to_string()
        + BANK_SUFFIX
        + "\n"
        + INSP_PREFIX
        + "infinite"
        + INSP_SUFFIX
}

fn build_roll_result(state: &InteractionState) -> String {
    let bet = state.game_state.bet();
    let bank = state.game_state.bank();

    if bet > bank {
        "You can't roll on more :shell:s than you have!\n".to_string() + &build_stats(bank)
    } else {
        let mut rng = thread_rng();
        let roll: u64 = rng.gen_range(0, 4);
        let winnings = roll * bet;
        let new_bank = bank - bet + winnings;
        format!(
            "# :woman_elf::slot_machine:
You rolled on {} :shell:s...
for a **{}x** multiplier.
You **won** {} :shell:s!\n",
            bet, roll, winnings
        ) + &build_stats(new_bank)
    }
}

fn build_free_result(state: &InteractionState) -> String {
    format!(
        "# :woman_elf::magic_wand:
You are given {} free :shell:s.
*Come again anytime!*\n",
        FREE_AMT
    ) + &build_stats(state.game_state.bank() + FREE_AMT)
}

fn translate_proof(hash: &[u8]) -> String {
    let mut proof = "".to_string();

    for i in 1..=PROOF_LENGTH {
        let n = hash[i];

        let prefix = n & 7;
        let space = n >> 3 & 1;

        proof += &match prefix {
            0 => "ba",
            1 => "la",
            2 => "ha",
            3 => "no",
            4 => "re",
            5 => "na",
            6 => "ne",
            _ => "sha",
        }
        .to_string();

        proof += &match space {
            0 => " ",
            _ => "",
        }
        .to_string();
    }

    proof.trim().to_string()
}

fn honorific(amt: u64) -> String {
    match amt {
        0 => "a :monkey: Blatant Bonobo :monkey:",
        1..=9 => "a :cucumber: Cool Cucumber :cucumber:",
        10..=49 => "a :cut_of_meat: Sizzlin' Steak :cut_of_meat:",
        50.. => "an :elf: Elegant Elf :elf:",
    }
    .to_string()
}

fn proof(id: &str, amt: &str) -> String {
    let s = SALT.to_string() + id + amt;
    let hash = <[u8; 32]>::from_hex(digest(s)).unwrap();
    translate_proof(&hash)
}

fn build_brag_result(state: &InteractionState) -> String {
    let id = &state.user;
    let bank = state.game_state.bank();

    format!(
        "## <@{}> has {} :shell:s!\n## <@{}> is {}\n",
        id,
        bank,
        id,
        honorific(bank)
    ) + &format!("### Proof: *{}*", proof(id, &bank.to_string()))
}

fn build_recall_submit_result(
    state: &InteractionState,
    fields: collections::HashMap<String, String>,
) -> String {
    format!(
        "inspecting claim of: {} and proof of: {}",
        fields.get("claim").unwrap(),
        fields.get("proof").unwrap()
    ) + &build_stats(state.game_state.bank() + FREE_AMT)
}

pub struct ShellsHandler;

impl Handler for ShellsHandler {
    fn handle_application_command(&self, _: &InteractionRequest) -> InteractionResponse {
        InteractionResponse::message()
            .content(&(build_rules_message() + "\n" + &build_stats(0)))
            .components(build_action_row())
            .into()
    }

    fn handle_message_component(&self, req: &InteractionRequest) -> InteractionResponse {
        let state: InteractionState = req.into();

        let id = req.custom_id().unwrap();

        let res: InteractionResponse = match id.as_str() {
            "roll" => InteractionResponse::message()
                .content(&build_roll_result(&state))
                .components(build_action_row())
                .into(),

            "free" => InteractionResponse::message()
                .content(&build_free_result(&state))
                .components(build_action_row())
                .into(),

            "brag" => InteractionResponse::message()
                .content(&build_brag_result(&state))
                .shout()
                .into(),

            "recall" => InteractionResponse::modal()
                .id("submit_recall")
                .title("Circle of Recall")
                .components(build_recall_fields())
                .into(),

            "rules" => InteractionResponse::message()
                .content(&(build_rules_message() + "\n" + &state.game_state.fmt()))
                .components(build_action_row())
                .into(),

            &_ => todo!(),
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

                let content = build_recall_submit_result(&state, req.modal_submit_values());

                InteractionResponse::message()
                    .content(&content)
                    .components(build_action_row())
                    .into()
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

        assert_eq!(state.bank() % 3043, 0);
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
            "# :woman_elf::magic_wand:\nYou are given 5 free :shell:s.\n*Come again anytime!*\n# Your Stats\nYou have: 3048 :shell:s\nYou have: infinite :zap:".to_string()
        );
    }
}
