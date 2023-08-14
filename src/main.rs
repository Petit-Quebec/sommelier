use lambda_http::{run, service_fn, Body, Error, Request, RequestExt, Response};
use std::env;
use serde_json::json;
use lambda_http::http::StatusCode;
mod app;
use crate::app::interactions::{InteractionRequest, InteractionResponse};
use crate::app::handle_interaction;

fn validate(event: &Request) -> Result<(), StatusCode> {

    match env::var("NYOOMIO_PUBLIC_KEY") {
        
        Ok(pk) => {
            
            let query_map = event.query_string_parameters_ref();
            
            let signature = query_map.and_then(|params| params.first("X-Signature-Ed25519"));
            
            let timestamp = query_map.and_then(|params| params.first("X-Signature-Timestamp"));
            
            if true {Ok(())} else {Err(StatusCode::UNAUTHORIZED)}
        },

        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
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

fn handle_request(event: &Request) -> Result<Response<Body>, StatusCode> {

    match std::str::from_utf8(event.body()) {
        
        Ok(s) => match handle_interaction_json(s) {

            Ok(resp) => Ok(build_interaction_response(resp)),

            Err(code) => Err(code)

        },

        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    
    match validate(&event) {
        
        Ok(())    => match handle_request(&event) {

            Ok(response) => Ok(response),

            Err(code) => Ok(build_error_response(code))
        },

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
