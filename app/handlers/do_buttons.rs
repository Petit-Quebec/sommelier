/*!
 * Implementation of "deedee" command.
 */

use crate::{Component, ComponentType, InteractionCallbackData, InteractionData, MessageFlags};

pub fn buttons(_: &InteractionData) -> InteractionCallbackData {

    let deedee_button = Component {
        r#type: ComponentType::Button,
        components: None,
        label: Some("deedee".to_string()),
    };

    let cgol_button = Component {
        r#type: ComponentType::Button,
        components: None,
        label: Some("game of life".to_string()),
    };

    InteractionCallbackData {
        content: Some("mega doo doo".to_string()),
        components: vec![
            Component {
                r#type: ComponentType::ActionRow,
                label: None,
                components: Some(vec![deedee_button, cgol_button]),
            }
        ],
        flags: Some(MessageFlags::Ephemeral),
    }
}
