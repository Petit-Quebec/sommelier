/*!
 * Implementation of "deedee" command.
 */

use crate::handlers::Handler;
use crate::{InteractionCallbackData, InteractionData, MessageFlags};

pub struct DeedeeHandler;

impl Handler for DeedeeHandler {
    fn handle_application_command(_: &InteractionData) -> InteractionCallbackData {
        InteractionCallbackData {
            content: Some("mega doo doo".to_string()),
            components: Vec::new(),
            flags: Some(MessageFlags::Ephemeral),
        }
    }
}
