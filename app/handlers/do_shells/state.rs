use crate::InteractionRequest;
use regex::Regex;
use std::fmt;

const BANK_PREFIX: &str = "You have:";
const BANK_SUFFIX: &str = ":shell:s";
const BET_PREFIX: &str = "You are betting:";
const BET_SUFFIX: &str = ":shell:s";
const INSP_PREFIX: &str = "You have:";
const INSP_SUFFIX: &str = ":squid:s";

pub struct InteractionState {
    pub user: String,
    pub game_state: GameState,
}

impl From<&InteractionRequest> for InteractionState {
    fn from(req: &InteractionRequest) -> Self {
        InteractionState {
            user: req.get_user(),
            game_state: (&req.message_content()).into(),
        }
    }
}

pub struct GameState {
    pub bet: u64,
    pub bank: u64,
    pub insp: u64,
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}\n{}\n{}\n",
            fmt_stat(BANK_PREFIX, self.bank, BANK_SUFFIX),
            fmt_stat(BET_PREFIX, self.bet, BET_SUFFIX),
            fmt_stat(INSP_PREFIX, self.insp, INSP_SUFFIX)
        )
    }
}

fn fmt_stat<T: fmt::Display>(prefix: &str, n: T, suffix: &str) -> String {
    vec![prefix, &n.to_string(), &suffix].join(" ")
}

impl From<&String> for GameState {
    fn from(msg: &String) -> Self {
        let bank = recognize_stat(msg, BANK_PREFIX, BANK_SUFFIX).unwrap_or(0);
        let bet = recognize_stat(msg, BET_PREFIX, BET_SUFFIX).unwrap_or(0);
        let insp = recognize_stat(msg, INSP_PREFIX, INSP_SUFFIX).unwrap_or(0);

        GameState {
            bet: bet,
            bank: bank,
            insp: insp,
        }
    }
}

fn recognize_stat(hay: &str, prefix: &str, suffix: &str) -> Option<u64> {
    let pattern = vec![prefix, &"[0-9]*".to_string(), suffix].join(" ");
    let re = Regex::new(&pattern).unwrap();
    let mut range = re.find(hay)?.range();
    range.start += prefix.len();
    range.end -= suffix.len();
    let n = hay[range].trim().parse::<u64>().ok()?;
    Some(n)
}
