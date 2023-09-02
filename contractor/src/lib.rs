use request_types::RegistrationRequest;
use reqwest::{Client, Response, Error};
use serde_json::json;

pub mod request_types;

const APP_ID: &str = env!("NYOOMIO_APPLICATION_ID");
const BOT_TOKEN: &str = env!("NYOOMIO_BOT_TOKEN");

pub async fn register_command(req: &RegistrationRequest) 
    -> Result<(), String> {

    let client = Client::new();

    let uri = format!("https://discord.com/api/v10/applications/{}/commands", APP_ID);

    let response = client.post(uri)
        .header("Authorization", "Bot ".to_owned() + BOT_TOKEN)
        .header("Content-Type", "application/json")
        .body(json!(req).to_string())
        .send()
        .await.map_err(|_| "Error making request".to_string())?;

    let text = response.text().await.map_err(|_| "err".to_string())?;

    Ok(())
}

