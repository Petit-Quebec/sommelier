/*!
 * Implementation of "deedee" command.
 */

use crate::{InteractionCallbackData, InteractionData, MessageFlags};

pub fn deedee(_: &InteractionData) -> InteractionCallbackData {
    InteractionCallbackData {
        content: Some("mega doo doo".to_string()),
        components: Vec::new(),
        flags: Some(MessageFlags::Ephemeral),
    }
}
