/*!
 * Discord interaction request and response types. These are serializable data structures that
 * match the JSON structure established by the Discord API.
 */

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Deserialize, PartialEq, Debug)]
pub struct InteractionRequest {
    pub id: String,
    pub application_id: String,
    pub r#type: InteractionType,
    pub data: Option<InteractionData>,
    pub guild_id: Option<String>,
    pub channel_id: Option<String>,
    pub member: Option<GuildMember>,
    pub message: Option<Message>,
}

#[derive(Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum InteractionType {
    Ping = 1,
    ApplicationCommand = 2,
    MessageComponent = 3,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct InteractionData {
    pub name: Option<String>,
    pub custom_id: Option<String>,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct GuildMember {
    pub user: Option<User>,
    pub nick: Option<String>,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct Message {
    pub content: String,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct User {
    pub id: String,
}

#[derive(PartialEq, Debug)]
pub struct InteractionMetadata<'a> {
    pub user_id: &'a String,
    pub channel_id: &'a String,
    pub guild_id: &'a String,
}

#[derive(Serialize, PartialEq, Debug)]
pub struct InteractionResponse {
    pub r#type: InteractionCallbackType,
    pub data: Option<InteractionCallbackData>,
}

#[derive(Serialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum InteractionCallbackType {
    Pong = 1,
    ChannelMessageWithSource = 4,
    UpdateMessage = 7,
}

#[derive(Serialize, PartialEq, Debug)]
pub struct InteractionCallbackData {
    pub content: Option<String>,
    pub flags: Option<MessageFlags>,
    pub components: Vec<ActionRow>,
}

#[derive(Serialize, PartialEq, Debug)]
pub struct ActionRow {
    pub r#type: ComponentType,
    pub components: Vec<Button>,
}

impl ActionRow {
    pub fn new() -> Self {
        ActionRow {
            r#type: ComponentType::ActionRow,
            components: Vec::new(),
        }
    }

    pub fn button(mut self, button: Button) -> Self {
        self.components.push(button);
        self
    }
}

#[derive(Serialize, PartialEq, Debug)]
pub struct Button {
    pub r#type: ComponentType,
    pub label: Option<String>,
    pub style: ButtonStyle,
    pub custom_id: String,
}

impl Button {
    pub fn new() -> Self {
        Button {
            r#type: ComponentType::Button,
            label: None,
            style: ButtonStyle::Primary,
            custom_id: "unlabeled button".to_string(),
        }
    }

    pub fn label(mut self, label: &str) -> Self {
        self.label = Some(label.to_string());
        self
    }

    pub fn id(mut self, id: &str) -> Self {
        self.custom_id = id.to_string();
        self
    }
}

#[derive(Serialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum ComponentType {
    ActionRow = 1,
    Button = 2,
}

#[derive(Serialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum ButtonStyle {
    Primary = 1,
}

#[derive(Serialize_repr, PartialEq, Debug)]
#[repr(u16)]
pub enum MessageFlags {
    Ephemeral = 64,
}
