/*!
 * Implementation of "deedee" command.
 */

use crate::{
    ActionRow, Button, ButtonStyle, ComponentType, InteractionCallbackData, InteractionData,
    MessageFlags,
};

pub fn buttons(_: &InteractionData) -> InteractionCallbackData {
    let cgol_button = Button {
        r#type: ComponentType::Button,
        style: ButtonStyle::Primary,
        label: Some("game of life".to_string()),
        custom_id: "cgol".to_string(),
    };

    let deedee_button = Button {
        r#type: ComponentType::Button,
        style: ButtonStyle::Primary,
        label: Some("deedee".to_string()),
        custom_id: "deedee".to_string(),
    };

    InteractionCallbackData {
        content: Some("here be buttons:".to_string()),
        components: vec![ActionRow {
            r#type: ComponentType::ActionRow,
            components: vec![deedee_button, cgol_button],
        }],
        flags: Some(MessageFlags::Ephemeral),
    }
}
