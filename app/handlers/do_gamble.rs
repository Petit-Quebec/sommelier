/*!
 * Implementation of "gamble" command.
 */

use crate::handlers::Handler;
use crate::{ActionRow, Button, InteractionRequest, InteractionResponse};
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
const INSP_SUFFIX: &str = " **inspiration**";
const PROOF_LENGTH: usize = 12;

fn build_action_row() -> ActionRow {
    let roll_button = Button::new().label("roll").id("roll");
    let free_button = Button::new().label("free").id("free");
    let brag_button = Button::new().label("brag").id("brag");
    let recall_button = Button::new().label("recall").id("recall");
    let rules_button = Button::new().label("rules").id("rules");

    ActionRow::new()
        .button(roll_button)
        .button(free_button)
        .button(brag_button)
        .button(recall_button)
        .button(rules_button)
}

fn build_recall_action_row() -> ActionRow {
    let set_claim_button = Button::new().label("set claim").id("set claim");
    let set_proof_button = Button::new().label("show proof").id("show proof");

    ActionRow::new()
        .button(set_claim_button)
        .button(set_proof_button)
}

fn build_rules_message() -> String {
    "# Welcome to Elf Gambling :woman_elf:

**Roll** to bet on your :shell:s, to receive 0x, 1x, 2x, or 3x the amount :shell:s back. There is a 25% chance of each of these happening.

Additionally, whenever you roll, you have a chance to gain **inspiration**. The more of your saved :shell:s you roll on, the higher the chance that you'll gain **inspiration**.

**Free** will give you a small number of :shell:s for free! No charge at all.

**Brag** will consume one **inspiration** to **brag** about your score. Let your friends know how many :shell:s you've got! When you brag, you'll also be provided with proof of your achievement in a **sselvish**, a dialect of cryptographically secure elvish. 

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
        format!("You rolled on {} :shell:s...\n", bet)
            + &format!("for a **{}**x multiplier!\n", roll)
            + &format!("You **won** {} :shell:s!\n", winnings)
            + &build_stats(new_bank)
    }
}

fn build_free_result(bank: u64) -> String {
    format!(
        "# :magic_wand:
You got {} free :shell:s. Come again anytime!\n",
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
    ) + &format!("**Proof**: *{}*", translate_proof(&hash))
}

fn build_recall_initiation(bank: u64) -> String {
    format!("# :leaves: :man_elf: :leaves:
Provide the amount you are claiming and the **sselvish** proof of your past achievement. Only then can the :man_elf: help you recall your past :shell:s.
:man_elf:: *By recalling your past achievement, you are leaving behind your current pool of {} :shell:s! If you're okay with that, we can proceed.*", bank)
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

pub struct GambleHandler;

impl Handler for GambleHandler {
    fn handle_application_command(&self, _: &InteractionRequest) -> InteractionResponse {
        InteractionResponse::new()
            .message(&(build_rules_message() + "\n" + &build_stats(0)))
            .component_row(build_action_row())
    }

    fn handle_message_component(&self, req: &InteractionRequest) -> InteractionResponse {
        let bank = recognize_bank(&req.message.as_ref().unwrap().content);

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
                .message(&build_roll_result(bank, bank))
                .component_row(build_action_row())
                .edit(),

            "free" => InteractionResponse::new()
                .message(&build_free_result(bank))
                .component_row(build_action_row())
                .edit(),

            "brag" => {
                let name = get_user_name(req);
                let msg = build_brag_result(&name, bank);
                InteractionResponse::new().message(&msg).shout()
            }

            "recall" => InteractionResponse::new()
                .message(&build_recall_initiation(bank))
                .component_row(build_recall_action_row())
                .edit(),

            "rules" => InteractionResponse::new()
                .message(&(build_rules_message() + "\n" + &build_stats(bank)))
                .component_row(build_action_row())
                .edit(),

            &_ => todo!(),
        }
    }
}
