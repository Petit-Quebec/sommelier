/*!
 * Implementation of "deedee" command.
 */

use crate::{
    ButtonStyle, Component, ComponentType, InteractionCallbackData, InteractionData, MessageFlags,
};

pub fn buttons(_: &InteractionData) -> InteractionCallbackData {
    let cgol_button = Component {
        r#type: ComponentType::Button,
        components: None,
        style: Some(ButtonStyle::Primary),
        label: Some("game of life".to_string()),
    };

    let deedee_button = Component {
        r#type: ComponentType::Button,
        components: None,
        style: Some(ButtonStyle::Primary),
        label: Some("deedee".to_string()),
    };

    InteractionCallbackData {
        content: Some("mega doo doo".to_string()),
        components: vec![Component {
            r#type: ComponentType::ActionRow,
            label: None,
            style: None,
            components: Some(vec![deedee_button, cgol_button]),
        }],
        flags: Some(MessageFlags::Ephemeral),
    }
}
