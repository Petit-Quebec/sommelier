/*!
 * Implementation of "gamble" command.
 */

use crate::handlers::Handler;
use crate::{ActionRow, Button, InteractionCallbackData, InteractionData, MessageFlags};

pub struct GambleHandler;

impl Handler for GambleHandler {
    fn handle_application_command(&self, _: &InteractionData) -> InteractionCallbackData {
        let roll_button = Button::new().label("roll").id("roll");

        let free_button = Button::new().label("free").id("free");

        let brag_button = Button::new().label("brag").id("brag");

        let help_button = Button::new().label("help").id("help");

        let action_row = ActionRow::new()
            .button(roll_button)
            .button(free_button)
            .button(brag_button)
            .button(help_button);

        InteractionCallbackData {
            content: Some("gambling stub".to_string()),
            components: vec![action_row],
            flags: Some(MessageFlags::Ephemeral),
        }
    }

    fn handle_message_component(&self, _: &InteractionData) -> InteractionCallbackData {
        InteractionCallbackData {
            content: Some("button reaction stub".to_string()),
            components: vec![],
            flags: Some(MessageFlags::Ephemeral),
        }
    }
}
