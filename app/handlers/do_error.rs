/*!
 * Implementation for returning an error.
 */

use crate::handlers::Handler;
use crate::{InteractionCallbackData, InteractionData, MessageFlags};

pub struct ErrorHandler;

impl Handler for ErrorHandler {
    fn handle_application_command(&self, _: &InteractionData) -> InteractionCallbackData {
        InteractionCallbackData {
            content: Some("Unknown command...".to_string()),
            components: Vec::new(),
            flags: Some(MessageFlags::Ephemeral),
        }
    }
}
