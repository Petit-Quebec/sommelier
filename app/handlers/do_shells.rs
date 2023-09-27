/*!
 * Implementation of "gamble" command.
 */

use crate::handlers::Handler;
use crate::{Component, InteractionRequest, InteractionResponse};
use hex::FromHex;
use rand::{thread_rng, Rng};
use regex::Regex;
use sha256::digest;

const SALT: &str = env!("SOMMELIER_GAMBLING_SALT");
const FREE_AMT: u64 = 5;
const STARTING_AMT: u64 = 0;
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

fn build_roll_result(bet: u64, bank: u64) -> String {
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

fn build_free_result(bank: u64) -> String {
    format!(
        "# :woman_elf::magic_wand:
You are given {} free :shell:s.
*Come again anytime!*\n",
        FREE_AMT
    ) + &build_stats(bank + FREE_AMT)
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

fn build_brag_result(id: &str, bank: u64) -> String {
    let s = SALT.to_string() + id + &bank.to_string();

    let hash = <[u8; 32]>::from_hex(digest(s)).unwrap();

    format!(
        "## <@{}> has {} :shell:s!\n## <@{}> is {}\n",
        id,
        bank,
        id,
        honorific(bank)
    ) + &format!("### Proof: *{}*", translate_proof(&hash))
}

fn build_recall_initiation(bank: u64) -> String {
    format!("# :woman_elf::leaves: Circle of Recall

Provide the number of :shell:s you are claiming and the **Sselvish** proof of your past achievement. Only then can you recall your past :shell:s.

*By recalling your past achievement, you are leaving behind your current pool of {} :shell:s! If you're okay with that, we can proceed.*", bank)
}

pub fn recognize_bank(hay: &str) -> u64 {
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

pub struct ShellsHandler;

impl Handler for ShellsHandler {
    fn handle_application_command(&self, _: &InteractionRequest) -> InteractionResponse {
        InteractionResponse::message()
            .content(&(build_rules_message() + "\n" + &build_stats(0)))
            .components(build_action_row())
            .into()
    }

    fn handle_message_component(&self, req: &InteractionRequest) -> InteractionResponse {
        let bank = recognize_bank(&req.message.as_ref().unwrap().content);

        let id = req.data.as_ref().unwrap().custom_id.as_ref().unwrap();

        let res: InteractionResponse = match id.as_str() {
            "roll" => InteractionResponse::message()
                .content(&build_roll_result(bank, bank))
                .components(build_action_row()),

            "free" => InteractionResponse::message()
                .content(&build_free_result(bank))
                .components(build_action_row()),

            "brag" => {
                let name = get_user_name(req);
                let msg = build_brag_result(&name, bank);
                InteractionResponse::message().content(&msg).shout()
            }

            "recall" => InteractionResponse::message().content(&build_recall_initiation(bank)),

            "rules" => InteractionResponse::message()
                .content(&(build_rules_message() + "\n" + &build_stats(bank)))
                .components(build_action_row()),

            &_ => todo!(),
        }
        .into();

        match id.as_str() {
            "brag" => res,
            _ => res.edit(),
        }
    }
}
