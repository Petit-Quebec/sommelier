use hex::FromHex;
use sha256::digest;

const SALT: Option<&str> = option_env!("SOMMELIER_GAMBLING_SALT");
const PROOF_LENGTH: usize = 12;

fn translate(hash: &[u8]) -> String {
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

pub fn proof(id: &str, amt: &str) -> String {
    let s = SALT.unwrap_or("SOME_DEFAULT_VALUE").to_string() + id + amt;
    let hash = <[u8; 32]>::from_hex(digest(s)).unwrap();
    translate(&hash)
}
