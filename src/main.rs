use ed25519_dalek::{Signature, Verifier, VerifyingKey, PUBLIC_KEY_LENGTH};
use lambda_http::http::StatusCode;
use lambda_http::{run, service_fn, Body, Error, Request, Response};
use serde_json::json;

use crate::app::handle_interaction;
use crate::app::interactions::InteractionRequest;

mod app;

/// Takes in a json request body, returns a json response body
fn handle_interaction_json(request_json: &str) -> Result<String, StatusCode> {
    tracing::info!({ %request_json }, "Handling request json");

    match serde_json::from_str::<InteractionRequest>(request_json) {
        Ok(interaction) => {
            let interaction_response = handle_interaction(&interaction);

            Ok(json!(interaction_response).to_string())
        }

        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

fn function_handler_helper(event: &Request) -> Result<String, StatusCode> {
    let application_public_key: [u8; PUBLIC_KEY_LENGTH] = hex::decode(&env!("NYOOMIO_PUBLIC_KEY"))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .try_into()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let verifier = VerifyingKey::from_bytes(&application_public_key)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let timestamp = event
        .headers()
        .get("X-Signature-Timestamp")
        .ok_or(StatusCode::BAD_REQUEST)?
        .to_str()
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let signature_str = event
        .headers()
        .get("X-Signature-Ed25519")
        .ok_or(StatusCode::BAD_REQUEST)?
        .to_str()
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let signature: [u8; 64] = hex::decode(&signature_str)
        .map_err(|_| StatusCode::BAD_REQUEST)?
        .try_into()
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let body = std::str::from_utf8(event.body()).map_err(|_| StatusCode::BAD_REQUEST)?;

    tracing::info!({ %body }, "Handling request json");

    let msg = (timestamp.to_owned() + body).into_bytes();

    match verifier.verify(&msg, &Signature::from_bytes(&signature)) {
        Ok(()) => Ok(handle_interaction_json(body)?),

        Err(_) => Err(StatusCode::UNAUTHORIZED),
    }
}

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    match function_handler_helper(&event) {
        Ok(response) => Ok(Response::builder()
            .status(StatusCode::OK)
            .body(response.into())
            .unwrap()),

        Err(code) => Ok(Response::builder()
            .status(code)
            .body("Error when handling request.".into())
            .unwrap()),
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_bad_body_json() {
        let req_json = "not json formatted";

        assert!(handle_interaction_json(req_json).is_err());
    }

    #[test]
    fn test_ping_ack_json() {
        let ping_json = r#"
        {
            "id": "my_id",
            "application_id": "app_id",
            "type": 1
        }
        "#;

        let expected_pong_json = r#"{"data":null,"type":1}"#;

        let pong_json = handle_interaction_json(ping_json);

        assert!(pong_json.is_ok());
        assert_eq!(
            handle_interaction_json(ping_json).expect(""),
            expected_pong_json
        );
    }
}
