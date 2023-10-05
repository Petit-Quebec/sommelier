use crate::InteractionRequest;
use regex::Regex;
use std::fmt;

const STAT_PREFIX: &str = "You have:";
const CURRENCY_SYMBOL: &str = ":shell:s";
const INSPIRATION_SYMBOL: &str = ":zap:";

pub struct InteractionState {
    pub user: String,
    pub game_state: GameState,
}

impl From<&InteractionRequest> for InteractionState {
    fn from(req: &InteractionRequest) -> Self {
        InteractionState {
            user: req.get_user(),
            game_state: req.message_content().into(),
        }
    }
}

pub struct GameState {
    bet: u64,
    bank: u64,
    inspiration: u64,
}

impl GameState {
    pub fn bank(&self) -> u64 {
        self.bank
    }

    pub fn bet(&self) -> u64 {
        self.bet
    }

    pub fn fmt(self) -> String {
        vec![
            fmt_stat(self.bank, CURRENCY_SYMBOL),
            fmt_stat(self.inspiration, INSPIRATION_SYMBOL),
        ]
        .join("\n")
    }
}

fn recognize_stat(hay: &str, symb: &str) -> Option<u64> {
    let pattern = vec![STAT_PREFIX, &"[0-9]*".to_string(), symb].join(" ");
    let re = Regex::new(&pattern).unwrap();
    let mut range = re.find(hay)?.range();
    range.start += STAT_PREFIX.len();
    range.end -= symb.len();
    let n = hay[range].trim().parse::<u64>().ok()?;
    Some(n)
}

impl From<String> for GameState {
    fn from(msg: String) -> Self {
        let bank = recognize_stat(&msg, CURRENCY_SYMBOL).unwrap_or(0);
        let insp = recognize_stat(&msg, INSPIRATION_SYMBOL).unwrap_or(0);

        GameState {
            bet: bank,
            bank: bank,
            inspiration: insp,
        }
    }
}

fn fmt_stat<T: fmt::Display>(n: T, symb: &str) -> String {
    vec![STAT_PREFIX, &n.to_string(), &symb].join(" ")
}
