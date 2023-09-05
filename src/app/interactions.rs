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
    pub user: Option<User>,
}

#[derive(Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum InteractionType {
    Ping = 1,
    ApplicationCommand = 2,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct InteractionData {
    pub name: String,
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
}

#[derive(Serialize, PartialEq, Debug)]
pub struct InteractionCallbackData {
    pub content: Option<String>,
}
