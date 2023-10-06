use crate::{Component, InteractionResponse};

pub fn plain_message(msg: &str) -> InteractionResponse {
    InteractionResponse::message()
        .content(msg)
        .components(build_action_row())
        .into()
}

pub fn quiet_message(msg: &str) -> InteractionResponse {
    plain_message(msg).edit()
}

pub fn loud_message(msg: &str) -> InteractionResponse {
    InteractionResponse::message().content(msg).shout().into()
}

pub fn recall_modal(id: &str, title: &str) -> InteractionResponse {
    InteractionResponse::modal()
        .id(id)
        .title(title)
        .components(build_recall_fields())
        .into()
}

pub fn set_roll_modal(id: &str, title: &str) -> InteractionResponse {
    InteractionResponse::modal()
        .id(id)
        .title(title)
        .components(build_set_roll_fields())
        .into()
}

fn build_action_row() -> Vec<Component> {
    let roll_button = Component::button().label("roll").id("roll").into();
    let set_roll_button = Component::button().label("set roll").id("set_roll").into();
    let free_button = Component::button().label("free").id("free").into();
    let brag_button = Component::button().label("brag").id("brag").into();
    let recall_button = Component::button().label("recall").id("recall").into();

    vec![
        roll_button,
        set_roll_button,
        free_button,
        brag_button,
        recall_button,
    ]
}

fn build_recall_fields() -> Vec<Component> {
    let claim = Component::text_input().label("claim").id("claim").into();
    let proof = Component::text_input().label("proof").id("proof").into();
    vec![claim, proof]
}

fn build_set_roll_fields() -> Vec<Component> {
    let roll_amt = Component::text_input()
        .label("Amount")
        .id("roll_amt")
        .into();
    vec![roll_amt]
}
