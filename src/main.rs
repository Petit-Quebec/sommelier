use ed25519_dalek::VerifyingKey;
use lambda_http::{run, service_fn, Body, Error, Request, RequestExt, Response};
use lambda_http::http::StatusCode;
use serde_json::json;
use std::env;

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
    
    match serde_json::from_str::<InteractionRequest>(request_json) {
        
        Ok(interaction) => {
            
            let interaction_response = handle_interaction(&interaction);

            Ok(json!(interaction_response).to_string())
        },

        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

fn function_handler_helper(event: &Request) -> Result<Response<Body>, StatusCode> {
    
    let application_public_key = lift_result(env::var("NYOOMIO_PUBLIC_KEY"))?;
    
    let timestamp = get_param(event, "X-Signature-Ed25519")?;

    let signature = get_param(event, "X-Signature-Timestamp")?;
   
    let body = lift_result(std::str::from_utf8(event.body()))?;

    let result_json = handle_interaction_json(body)?;

    Ok(build_interaction_response(result_json))
}

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {

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

