/*!
 * Manages the interaction contract between the Discord client and the application backend. Adds
 * new commands to this contract when requested.
 */

use request_types::RegistrationRequest;
use reqwest::Client;
use serde_json::json;

pub mod request_types;

const APPLICATION_ID: &str = env!("PROSPECTOR_APPLICATION_ID");
const BOT_TOKEN: &str = env!("PROSPECTOR_BOT_TOKEN");

pub async fn register_command(req: &RegistrationRequest) -> Result<(), String> {
    let client = Client::new();

    let uri = format!(
        "https://discord.com/api/v10/applications/{}/commands",
        APPLICATION_ID
    );

    let response = client
        .post(uri)
        .header("Authorization", "Bot ".to_owned() + BOT_TOKEN)
        .header("Content-Type", "application/json")
        .body(json!(req).to_string())
        .send()
        .await
        .map_err(|_| "Error making request".to_string())?;

    response
        .text()
        .await
        .map_err(|_| "Error returned by request".to_string())?;

    Ok(())
}
