use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/*
 * Request Types
 */
#[derive(Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum InteractionType {
    Ping = 1,
    ApplicationCommand = 2,
    MessageComponent = 3,
    ApplicationCommandAutocomplete = 4,
    ModalSubmit = 5,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct InteractionData {
    pub name: String,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct InteractionRequest {
    pub id: String,
    pub application_id: String,
    pub r#type: InteractionType,
    pub data: Option<InteractionData>,
}

#[derive(PartialEq, Debug)]
pub struct InteractionMetadata {
    pub user_id: Option<String>,
    pub channel_id: Option<String>,
    pub guild_id: Option<String>,
}

/*
 * Response Types
 */
#[derive(Serialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum InteractionCallbackType {
    Pong = 1,
    ChannelMessageWithSource = 4,
    DeferredChannelMessageWithSource = 5,
    DefferedUpdateMessage = 6,
    UpdateMessage = 7,
    ApplicationCommandAutocompleteResult = 8,
    Modal = 9,
}

#[derive(Serialize, PartialEq, Debug)]
pub struct InteractionCallbackData {
    pub content: Option<String>,
}

#[derive(Serialize, PartialEq, Debug)]
pub struct InteractionResponse {
    pub r#type: InteractionCallbackType,
    pub data: Option<InteractionCallbackData>,
}
