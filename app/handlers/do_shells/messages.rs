use super::InteractionState;

fn build_stats(state: &InteractionState) -> String {
    "## Your Stats\n".to_string() + &state.game_state.to_string()
}

pub fn welcome_message(state: &InteractionState) -> String {
    r#"# :woman_elf::shell: Shell Game :shell:

:game_die: **Roll** will roll on your :shell:s, to receive 0x, 1x, 2x, or 3x the amount of :shell:s back.

:bubbles: **Set Roll** allows you to set the amount of :shell:s you want to roll.

:beach: **Free** will give you a small number of :shell:s for free. Or, you could get a :star2:...

:trumpet: **Brag** will consume a :star2: to **brag** about your score. Let your friends know how many :shell:s you've got! When you brag, you'll also be provided with proof of your achievement in **Sselvish**, a cryptographically secure dialect of Common Elvish.

:leaves: **Recall** allows you reset your current gambling run to a past gambling run that you **bragged** about.
"#.to_string()
 + &build_stats(state)
}

pub fn roll_success_message(bet: u64, roll: u64, state: &InteractionState) -> String {
    format!(
        r#"# :woman_elf::game_die: Shell Roll :game_die:

You rolled on {} :shell:s...

and got a **{}x** multiplier.

You **won** {} :shell:s!
"#,
        bet,
        roll,
        bet * roll
    ) + &build_stats(state)
}

pub fn roll_failure_message(state: &InteractionState) -> String {
    r#"# :woman_elf::game_die: Shell Roll :game_die:

You can't roll on more :shell:s than you have!
"#
    .to_string()
        + &build_stats(state)
}

pub fn set_roll_success_message(bet: u64, state: &InteractionState) -> String {
    format!(
        r#"# :bubbles: Set Roll :bubbles:

You set your roll amount to {}.
"#,
        bet,
    ) + &build_stats(state)
}

pub fn set_roll_amt_failure_message(state: &InteractionState) -> String {
    r#"# :bubbles: Set Roll :bubbles:

You can't try to roll more than you have in your bank!
"#
    .to_string()
        + &build_stats(state)
}

pub fn set_roll_parse_failure_message(state: &InteractionState) -> String {
    r#"# :bubbles: Set Roll :bubbles:

You can only set your roll to a number!
"#
    .to_string()
        + &build_stats(state)
}

pub fn free_message(gain: Option<u64>, insp: Option<u64>, state: &InteractionState) -> String {
    r#"# :beach: Shimmering Sands :beach:
"#
    .to_string()
        + &match gain {
            Some(g) => {
                "You sift through the sands to find ".to_string() + &g.to_string() + " :shell:s.\n"
            }
            None => "".to_string(),
        }
        + &match insp {
            Some(i) => {
                "A glimmer in the sand catches your eye. Upon further inspection, you find "
                    .to_string()
                    + &i.to_string()
                    + " :star2:s!\n"
            }
            None => "".to_string(),
        }
        + &build_stats(state)
}

pub fn proof_success_message(proof: &str, state: &InteractionState) -> String {
    let user = &state.user;
    let bank = state.game_state.bank;
    format!(
        r#"# :scroll: The Scribe :scroll:

Let it be noted to the public that:
> <@{}> has {} :shell:s!
> <@{}> is a {}!
### Proof: *{}*
"#,
        user,
        bank,
        user,
        honorific(bank),
        proof
    ) + &build_stats(state)
}

pub fn proof_failure_message(state: &InteractionState) -> String {
    r#":scroll: The Scribe :scroll:

The Scribe cannot provide proof of your deed without a :squid:!

You can find :squid:s at the **beach**!
"#
    .to_string()
        + &build_stats(state)
}

fn honorific(bank: u64) -> String {
    match bank {
        0 => "a :monkey: Blatant Bonobo :monkey:",
        1..=9 => "a :cucumber: Cool Cucumber :cucumber:",
        10..=49 => "a :cut_of_meat: Sizzlin' Steak :cut_of_meat:",
        50.. => "an :elf: Elegant Elf :elf:",
    }
    .to_string()
}

pub fn recall_success_message(proof: &str, state: &InteractionState) -> String {
    format!(
        r#"# :leaves: Circle of Recall :leaves:

You utter your **Sselvish** proof: *{}*. 

Your claim is legitimate! You recall {} :shell:s!
"#,
        proof, state.game_state.bank
    ) + &build_stats(state)
}

pub fn recall_failure_message(proof: &str, state: &InteractionState) -> String {
    format!(
        r#"# :leaves: Circle of Recall :leaves:

You utter your **Sselvish** proof: *{}*. 

Your claim fails! You cannot recall anything.
"#,
        proof
    ) + &build_stats(state)
}
