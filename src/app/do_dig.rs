/*!
 * Implementation of "dig" command.
 */

use crate::app::InteractionCallbackData;
use crate::app::InteractionMetadata;

pub fn dig(metadata: &InteractionMetadata) -> InteractionCallbackData {
    InteractionCallbackData {
        content: Some(format!(
            "u: {}, c: {}, g: {}",
            metadata.user_id, metadata.channel_id, metadata.guild_id
        )),
    }
}
