/*!
 * Implementation of "dig" command.
 */

use crate::app::hash_location;
use crate::app::InteractionCallbackData;
use crate::app::InteractionMetadata;

pub fn dig(metadata: &InteractionMetadata) -> InteractionCallbackData {
    InteractionCallbackData {
        content: Some(hash_location(metadata.channel_id, metadata.guild_id)),
    }
}
