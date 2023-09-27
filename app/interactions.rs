/*!
 * Discord interaction request and response types. These are serializable data structures that
 * match the JSON structure established by the Discord API.
 */

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Deserialize, PartialEq, Debug)]
pub struct InteractionRequest {
    pub r#type: InteractionType,
    pub data: Option<InteractionData>,
    pub member: Option<GuildMember>,
    pub message: Option<Message>,
}

impl InteractionRequest {
    pub fn user(&self) -> String {
        match &self.member {
            Some(m) => m.user.id.clone(),
            None => "Unknown user".to_string(),
        }
    }

    pub fn message(&self) -> String {
        match &self.message {
            Some(m) => m.content.clone(),

            None => "".to_string(),
        }
    }
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
    pub user: User,
    pub nick: Option<String>,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct Message {
    pub content: String,
    pub interaction: Option<MessageInteraction>,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct MessageInteraction {
    pub name: String,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct User {
    pub id: String,
}

#[derive(Serialize, PartialEq, Debug)]
pub struct InteractionResponse {
    r#type: InteractionCallbackType,
    data: InteractionCallbackData,
}

impl InteractionResponse {
    pub fn pong() -> Self {
        let data = MessageCallbackData {
            content: "".to_string(),
            flags: None,
            components: Vec::new(),
        };

        InteractionResponse {
            r#type: InteractionCallbackType::Pong,
            data: InteractionCallbackData::Message(data),
        }
    }

    pub fn message() -> MessageCallbackData {
        MessageCallbackData {
            content: "".to_string(),
            flags: Some(MessageFlags::Ephemeral),
            components: Vec::new(),
        }
    }

    pub fn edit(mut self) -> Self {
        self.r#type = InteractionCallbackType::UpdateMessage;
        self
    }

    pub fn message_content(&self) -> Option<String> {
        match &self.data {
            InteractionCallbackData::Message(m) => Some(m.content.clone()),
            _ => None,
        }
    }

    pub fn message_components(&self) -> Vec<Component> {
        match &self.data {
            InteractionCallbackData::Message(m) => {
                if m.components.len() != 1 {
                    panic!();
                } else {
                    m.components[0].components.clone()
                }
            }
            _ => panic!(),
        }
    }
}

impl From<ModalCallbackData> for InteractionResponse {
    fn from(data: ModalCallbackData) -> InteractionResponse {
        InteractionResponse {
            r#type: InteractionCallbackType::Modal,
            data: InteractionCallbackData::Modal(data),
        }
    }
}

impl From<MessageCallbackData> for InteractionResponse {
    fn from(data: MessageCallbackData) -> InteractionResponse {
        InteractionResponse {
            r#type: InteractionCallbackType::ChannelMessageWithSource,
            data: InteractionCallbackData::Message(data),
        }
    }
}

#[derive(Serialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum InteractionCallbackType {
    Pong = 1,
    ChannelMessageWithSource = 4,
    UpdateMessage = 7,
    Modal = 9,
}

#[derive(Serialize, PartialEq, Debug)]
#[serde(untagged)]
pub enum InteractionCallbackData {
    Message(MessageCallbackData),
    Modal(ModalCallbackData),
}

#[derive(Serialize, PartialEq, Debug)]
pub struct MessageCallbackData {
    content: String,
    flags: Option<MessageFlags>,
    components: Vec<ActionRow>,
}

impl MessageCallbackData {
    pub fn content(mut self, msg: &str) -> Self {
        self.content = msg.to_string();
        self
    }

    pub fn components(mut self, components: Vec<Component>) -> Self {
        self.components = vec![ActionRow::new().components(components)];
        self
    }

    pub fn shout(mut self) -> Self {
        self.flags = None;
        self
    }
}

#[derive(Serialize, PartialEq, Debug)]
pub struct ModalCallbackData {
    custom_id: String,
    title: String,
    components: Vec<Component>,
}

#[derive(Serialize, PartialEq, Debug)]
struct ActionRow {
    r#type: ComponentType,
    components: Vec<Component>,
}

impl ActionRow {
    pub fn new() -> Self {
        ActionRow {
            r#type: ComponentType::ActionRow,
            components: Vec::new(),
        }
    }

    pub fn components(mut self, components: Vec<Component>) -> Self {
        self.components = components;
        self
    }
}

#[derive(Serialize, PartialEq, Debug, Clone)]
#[serde(untagged)]
pub enum Component {
    Button(Button),
    Text(TextInput),
}

impl Component {
    pub fn button() -> Button {
        Button {
            r#type: ComponentType::Button,
            label: None,
            style: ButtonStyle::Primary,
            custom_id: "unlabeled button".to_string(),
        }
    }
}

impl From<Button> for Component {
    fn from(button: Button) -> Component {
        Component::Button(button)
    }
}

#[derive(Serialize, PartialEq, Debug, Clone)]
pub struct Button {
    r#type: ComponentType,
    label: Option<String>,
    style: ButtonStyle,
    custom_id: String,
}

impl Button {
    pub fn label(mut self, label: &str) -> Self {
        self.label = Some(label.to_string());
        self
    }

    pub fn id(mut self, id: &str) -> Self {
        self.custom_id = id.to_string();
        self
    }
}

#[derive(Serialize, PartialEq, Debug, Clone)]
pub struct TextInput {
    r#type: ComponentType,
    label: String,
    style: TextInputStyle,
    custom_id: String,
}

impl TextInput {
    pub fn new() -> Self {
        TextInput {
            r#type: ComponentType::TextInput,
            label: "".to_string(),
            style: TextInputStyle::Short,
            custom_id: "unlabeled text input".to_string(),
        }
    }

    pub fn label(mut self, label: &str) -> Self {
        self.label = label.to_string();
        self
    }

    pub fn id(mut self, id: &str) -> Self {
        self.custom_id = id.to_string();
        self
    }
}

#[derive(Serialize, PartialEq, Debug, Clone)]
enum TextInputStyle {
    Short,
}

#[derive(Serialize_repr, PartialEq, Debug, Clone)]
#[repr(u8)]
enum ComponentType {
    ActionRow = 1,
    Button = 2,
    TextInput = 4,
}

#[derive(Serialize_repr, PartialEq, Debug, Clone)]
#[repr(u8)]
enum ButtonStyle {
    Primary = 1,
}

#[derive(Serialize_repr, PartialEq, Debug)]
#[repr(u16)]
enum MessageFlags {
    Ephemeral = 64,
}
