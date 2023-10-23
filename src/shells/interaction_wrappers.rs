use discord_interaction::{Component, Response};

pub fn new_message(msg: &str) -> Response {
    Response::message()
        .content(msg)
        .components(build_action_row())
        .shout()
        .into()
}

pub fn edit_message(msg: &str) -> Response {
    new_message(msg).edit()
}

pub fn recall_modal(id: &str, title: &str) -> Response {
    Response::modal()
        .id(id)
        .title(title)
        .components(build_recall_fields())
        .into()
}

pub fn set_roll_modal(id: &str, title: &str) -> Response {
    Response::modal()
        .id(id)
        .title(title)
        .components(build_set_roll_fields())
        .into()
}

fn build_action_row() -> Vec<Component> {
    let roll_button = Component::button().label("roll").id("roll").into();
    let set_roll_button = Component::button().label("set").id("set_roll").into();
    let free_button = Component::button().label("free").id("free").into();
    let proof_button = Component::button().label("proof").id("proof").into();
    let recall_button = Component::button().label("recall").id("recall").into();

    vec![
        roll_button,
        set_roll_button,
        free_button,
        proof_button,
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
