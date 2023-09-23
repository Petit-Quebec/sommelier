/*!
 * Implementation of "gamble" command.
 */

use crate::{InteractionCallbackData, InteractionData, MessageFlags};

pub fn gamble() -> InteractionCallbackData {
    InteractionCallbackData {
        content: Some("gambling stub".to_string()),
        components: vec![],
        flags: Some(MessageFlags::Ephemeral),
    }
}
