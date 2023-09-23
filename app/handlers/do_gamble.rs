/*!
 * Implementation of "gamble" command.
 */

use crate::{ActionRow, Button, InteractionCallbackData, MessageFlags};

pub fn gamble() -> InteractionCallbackData {
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
