/*!
 * Hashing logic for mapping game locations to game values.
 */

use sha256::digest;

const SALT: &str = env!("PROSPECTOR_SALT");

pub fn hash_location(channel_id: &String, guild_id: &String) -> String {
    digest(SALT.to_string() + channel_id + guild_id)
}
