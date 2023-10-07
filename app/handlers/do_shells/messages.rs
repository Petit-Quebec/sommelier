use super::InteractionState;

fn build_stats(state: &InteractionState) -> String {
    "## Your Stats\n".to_string() + &state.game_state.to_string()
}

pub fn welcome_message(state: &InteractionState) -> String {
    r#"# :woman_elf::shell: Shell Game :shell:

**Roll** to bet your :shell:s, to receive 0x, 1x, 2x, or 3x the amount :shell:s back.

**Set Roll* to choose the amount of :shell:s you want to roll.

**Free** to give you a small number of :shell:s for free. Or, you could get a :star2:...

**Brag** will consume a :star2: to **brag** about your score. Let your friends know how many :shell:s you've got! When you brag, you'll also be provided with proof of your achievement in **Sselvish**, a cryptographically secure dialect of Common Elvish.

**Recall** allows you to reset your current gambling run to a past gambling run that you **bragged** about.
"#.to_string()
 + &build_stats(state)
}

pub fn roll_success_message(bet: u64, roll: u64, state: &InteractionState) -> String {
    format!(
        r#"# :woman_elf::slot_machine:

You rolled on {} :shell:s...

for a **{}x** multiplier.

You **won** {} :shell:s!
"#,
        bet,
        roll,
        bet * roll
    ) + &build_stats(state)
}

pub fn roll_failure_message(state: &InteractionState) -> String {
    r#"You can't roll on more :shell:s than you have!
"#
    .to_string()
        + &build_stats(state)
}

pub fn set_roll_success_message(bet: u64, state: &InteractionState) -> String {
    format!(
        r#"# :game_die:

Your new roll amount is *{}*.
"#,
        bet,
    ) + &build_stats(state)
}

pub fn set_roll_amt_failure_message(state: &InteractionState) -> String {
    r#"# :game_die:

You can't try to roll to be more than you have in your bank!
"#
    .to_string()
        + &build_stats(state)
}

pub fn set_roll_parse_failure_message(state: &InteractionState) -> String {
    r#"# :game_die:

You have to set your roll to a number!
"#
    .to_string()
        + &build_stats(state)
}

pub fn free_message(gain: Option<u64>, insp: Option<u64>, state: &InteractionState) -> String {
    r#"# :beach:
"#
    .to_string()
        + &match gain {
            Some(g) => "You find ".to_string() + &g.to_string() + " :shell:s.\n",
            None => "".to_string(),
        }
        + &match insp {
            Some(i) => "You gain ".to_string() + &i.to_string() + " :star2:s.\n",
            None => "".to_string(),
        }
        + &build_stats(state)
}

pub fn brag_message(proof: &str, state: &InteractionState) -> String {
    let user = &state.user;

    let bank = state.game_state.bank;

    format!(
        r#"## <@{}> has {} :shell:s!

## <@{}> is {}!

### Proof: *{}*"#,
        user,
        bank,
        user,
        honorific(bank),
        proof
    )
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
        r#"# Circle of Recall

You utter your **Sselvish** proof: *{}*. 

Your claim is legitimate! You recall {} :shell:s!
"#,
        proof, state.game_state.bank
    ) + &build_stats(state)
}

pub fn recall_failure_message(proof: &str, state: &InteractionState) -> String {
    format!(
        r#"# Circle of Recall

You utter your **Sselvish** proof: *{}*. 

Your claim fails! You cannot recall anything.
"#,
        proof
    ) + &build_stats(state)
}
