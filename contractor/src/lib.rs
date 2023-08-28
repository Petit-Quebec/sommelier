use request_types::RegistrationRequest;
use reqwest::{Client, Response, Error};

pub mod request_types;

const APP_ID: &str = env!("NYOOMIO_APPLICATION_ID");

pub async fn register_command(req: &RegistrationRequest) 
    -> Result<(), String> {

    let client = Client::new();

    let uri = format!("https://discord.com/api/v10/applications/{}/commands", APP_ID);

    let response = client.post(uri)
        .body("the exact body that is setn")
        .send()
        .await.map_err(|_| "Error making request".to_string())?;

    let text = response.text().await.map_err(|_| "err".to_string())?;
    println!("{}", text);

    Ok(())
}

