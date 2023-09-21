/*!
 * Implementation of "deedee" command.
 */

use crate::{
    ActionRow, Button, ButtonStyle, ComponentType, InteractionCallbackData, InteractionData,
    MessageFlags,
};

pub fn buttons(_: &InteractionData) -> InteractionCallbackData {
    set_up_buttons("0")
}

pub fn buttons_plus_one(old: &str) -> InteractionCallbackData {
    let new = (old.parse::<i32>().unwrap_or(0) + 1).to_string();
    set_up_buttons(&new)
}

pub fn buttons_minus_one(old: &str) -> InteractionCallbackData {
    let new = (old.parse::<i32>().unwrap_or(0) - 1).to_string();
    set_up_buttons(&new)
}

fn set_up_buttons(text: &str) -> InteractionCallbackData {
    let plus_one = Button {
        r#type: ComponentType::Button,
        style: ButtonStyle::Primary,
        label: Some("+1".to_string()),
        custom_id: "buttons_+1".to_string(),
    };

    let minus_one = Button {
        r#type: ComponentType::Button,
        style: ButtonStyle::Primary,
        label: Some("-1".to_string()),
        custom_id: "buttons_-1".to_string(),
    };

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
        content: Some(text.to_string()),
        components: vec![ActionRow {
            r#type: ComponentType::ActionRow,
            components: vec![plus_one, minus_one, deedee_button, cgol_button],
        }],
        flags: Some(MessageFlags::Ephemeral),
    }
}
