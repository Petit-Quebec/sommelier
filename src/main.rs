use lambda_http::{run, service_fn, Body, Error, Request, RequestPayloadExt, RequestExt, Response};
use std::env;
use lambda_http::http::StatusCode;
use lambda_http::Body::Text;
mod discord_types;

/// Takes in a request, validates it, providing an appropriate error code
/// if validation was unsuccessful.
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



fn handle_request(event: &Request) -> Response<Body> {
    
    let body = Text("hi".to_string());
    let payload = event.payload;
    //let payload = event.payload::<discord_types::DiscordRequest>();
    match payload {
        Ok(_) => {println!("recognized payload");},
        Err(_) => {println!("parse error");}
    }
    Response::builder()
        .status(StatusCode::OK)
        .body(body)
        .unwrap()
}

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {

    match validate(&event) {
        Ok(())    => Ok(handle_request(&event)),
        Err(code) => {
            let response = Response::builder()
                .status(code)
                .body("Error when authenticating request.".into()).unwrap();
            Ok(response)
        }
    }

    /*
    // Extract some useful information from the request
    let who = event
        .query_string_parameters_ref()
        .and_then(|params| params.first("name"))
        .unwrap_or("world");

    let message = format!("Helllo , this is an AWS Lambda HTTP request");

    // Return something that implements IntoResponse.
    // It will be serialized to the right response event automatically by the runtime
    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(message.into())
        .map_err(Box::new)?;
    Ok(resp)*/
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
