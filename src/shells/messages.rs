use super::InteractionState;

fn build_stats(state: &InteractionState) -> String {
    "## Your Stats\n".to_string() + &state.game_state.to_string()
}

pub fn welcome_message(state: &InteractionState) -> String {
    r#"# :woman_elf: Shell Game :woman_elf:

:game_die: **Roll** will roll on your :shell:s, to receive 0x, 1x, 2x, or 3x the amount of :shell:s back.

:abacus: **Set** allows you to set the amount of :shell:s you want to roll.

:beach: **Free** will give you a small number of :shell:s for free. You could even get a :squid:...

:scroll: **Proof** will consume a :squid: to create a record of your winnings. This record will include proof of your achievement in **Sselvish**, a cryptographically secure dialect of Common Elvish.

:wind_blowing_face: **Recall** allows you set your current :shell:s to a past amount of :shell:s, provided you have **proof** of that achievement.
"#.to_string()
 + &build_stats(state)
}

pub fn roll_success_message(bet: u64, roll: u64, state: &InteractionState) -> String {
    format!(
        r#"# :game_die: Roll the Dice! :game_die:

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
    r#"# :game_die: Roll the Dice! :game_die:

You can't roll on more :shell:s than you have!
"#
    .to_string()
        + &build_stats(state)
}

pub fn set_roll_success_message(bet: u64, state: &InteractionState) -> String {
    format!(
        r#"# :abacus: Crunching Numbers :abacus:

You set your roll amount to {}.
"#,
        bet,
    ) + &build_stats(state)
}

pub fn set_roll_amt_failure_message(state: &InteractionState) -> String {
    r#"# :abacus: Crunching Numbers :abacus:

You can't try to roll more than you have in your bank!
"#
    .to_string()
        + &build_stats(state)
}

pub fn set_roll_parse_failure_message(state: &InteractionState) -> String {
    r#"# :abacus: Crunching Numbers :abacus:

You can only set your roll to a number!
"#
    .to_string()
        + &build_stats(state)
}

pub fn free_message(gain: Option<u64>, insp: Option<u64>, state: &InteractionState) -> String {
    r#"# :beach: Tidepools :beach:
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
                    + " :squid:s!\n"
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

*Make sure to record your :shell: amount and its **proof**. The Scribe is not a recordkeeper!*
"#,
        user,
        bank,
        user,
        honorific(bank),
        proof
    ) + &build_stats(state)
}

pub fn proof_failure_message(state: &InteractionState) -> String {
    r#"# :scroll: The Scribe :scroll:

The Scribe cannot provide proof of your deed without a :squid:!

You can find :squid:s at the **beach**!
"#
    .to_string()
        + &build_stats(state)
}

fn honorific(bank: u64) -> String {
    match bank {
        // not sure how to do this idiomatically...
        35184372088832.. => "a :cloud_tornado: Turbulent Twister :cloud_tornado:",
        4398046511104.. => "a :cloud: Camouflaged Cloud :cloud:",
        549755813888.. => "a :ocean: Whopping Wave :ocean:",
        68719476736.. => "a :bubbles: Brilliant Bubble :bubbles:",
        8589934592.. => "a :microbe: Mysterious Microbe :microbe:",
        1073741824.. => "a :worm: Wriggling Worm :worm:",
        134217728.. => "a :coral: Eef Reef Feef Reef :coral:",
        16777216.. => "a :shrimp: Shiny Shrimp :shrimp:",
        2097152.. => "a :coconut: Creamy Coconut :coconut:",
        262144.. => "a :crab: Crude Crab :crab:",
        32768.. => "an :octopus: Obscure Octopus :octopus:",
        4096.. => "a :lobster: Lovely Lobster :lobster:",
        512.. => "a :cucumber: Cool Cucumber :cucumber:",
        64.. => "a :seal: Slippery Seal :seal:",
        8.. => "a :parrot: Petulant Parrot :parrot:",
        0.. => "a :monkey: Blatant Bonobo :monkey:",
    }
    .to_string()
}

pub fn recall_success_message(proof: &str, state: &InteractionState) -> String {
    format!(
        r#"# :wind_blowing_face: Circle of Recall :wind_blowing_face:

You utter your **Sselvish** proof: *{}*. 

Your claim is legitimate! You recall {} :shell:s!
"#,
        proof, state.game_state.bank
    ) + &build_stats(state)
}

pub fn recall_failure_message(proof: &str, state: &InteractionState) -> String {
    format!(
        r#"# :wind_blowing_face: Circle of Recall :wind_blowing_face:

You utter your **Sselvish** proof: *{}*. 

Your claim fails! You cannot recall anything.
"#,
        proof
    ) + &build_stats(state)
}
