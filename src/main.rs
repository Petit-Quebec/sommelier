use ed25519_dalek::{Signature, Verifier, VerifyingKey, PUBLIC_KEY_LENGTH};
use hex::decode;
use lambda_http::{run, service_fn, Body, Error, Request, RequestExt, Response};
use lambda_http::http::StatusCode;
use serde_json::json;

use crate::app::interactions::{InteractionRequest};
use crate::app::handle_interaction;

mod app;

fn lift_option<T>(opt: Option<T>) -> Result<T, StatusCode> {

    match opt {

        Some(val) => Ok(val),

        None => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

fn lift_result<T, E>(res: Result<T, E>) -> Result<T, StatusCode> {

    lift_option(res.ok())
}

fn get_param(event: &Request, key: &str) -> Result<String, StatusCode> {

    tracing::info!({%key}, "accessing param");
    let query_map = event.query_string_parameters_ref();
    
    let val = query_map.and_then(|params| params.first(key));

    lift_option(val.map(|s| s.to_string()))
}

fn build_interaction_response(response: String) -> Response<Body> {
    
    Response::builder()
        .status(StatusCode::OK)
        .body(response.into())
        .unwrap()
}

fn build_error_response(code: StatusCode) -> Response<Body> {
    
    Response::builder()
        .status(code)
        .body("Error when handling request.".into())
        .unwrap()
}

/// Takes in a json request body, returns a json response body
fn handle_interaction_json(request_json: &str) -> Result<String, StatusCode> {
    
    tracing::info!({ %request_json }, "Handling request json");
    
    match serde_json::from_str::<InteractionRequest>(request_json) {
        
        Ok(interaction) => {
            
            let interaction_response = handle_interaction(&interaction);

            Ok(json!(interaction_response).to_string())
        },

        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

fn decode_hex_string(s: &str) -> Result<Vec<u8>, StatusCode> {

    lift_result(hex::decode(s))
}

fn function_handler_helper(event: &Request) -> Result<Response<Body>, StatusCode> {
    
    tracing::info!("Got to handler helper");
    
    let application_public_key_str = env!("NYOOMIO_PUBLIC_KEY");

    tracing::info!({%application_public_key_str}, "got public key");

    let apk = decode_hex_string(&application_public_key_str)?;

    tracing::info!("Decoded hex string!");

    let application_public_key: [u8; PUBLIC_KEY_LENGTH] = 
        lift_result(apk.try_into())?;

    tracing::info!("application public key");
    let verifier = lift_result(VerifyingKey::from_bytes(&application_public_key))?;
    
    tracing::info!("CCC");
    let timestamp_val = lift_option(event.headers().get("X-Signature-Timestamp"))?;
    let timestamp = lift_result(timestamp_val.to_str())?;

    tracing::info!("DDD");
    let signature_val = lift_option(event.headers().get("X-Signature-Ed25519"))?;
    let signature = lift_result(signature_val.to_str())?;

    tracing::info!({ %signature }, "Got signature");
    tracing::info!("EEE");
    let sb = lift_result(decode_hex_string(&signature))?;

    let signature_bytes: [u8; 64] = 
        lift_result(sb.try_into())?;

    tracing::info!("FFF");
    let body = lift_result(std::str::from_utf8(event.body()))?;

    tracing::info!({ %body }, "Handling request json");
    tracing::info!({ %signature }, "Got signature");
    
    let msg = (timestamp.to_owned() + body).into_bytes();

    match verifier.verify(&msg, &Signature::from_bytes(&signature_bytes)) {

        Ok(()) => {

            let result_json = handle_interaction_json(body)?;

            Ok(build_interaction_response(result_json))
        },

        Err(_) => Err(StatusCode::UNAUTHORIZED)
    }
}

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {

    tracing::info!("HANDLING INCOMING EVENT");

    match function_handler_helper(&event) {

        Ok(result) => Ok(result),

        Err(code) => Ok(build_error_response(code))
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

        let expected_pong_json = r#"{"type":1}"#;
        
        let pong_json = handle_interaction_json(ping_json);

        assert!(pong_json.is_ok());
        assert_eq!(handle_interaction_json(ping_json).expect(""), expected_pong_json);
    }
}

