use contractor::register_command;
use contractor::request_types::RegistrationRequest;

#[tokio::main]
async fn main() {
    // This is where we will setup our HTTP client requests.

    let request = RegistrationRequest {
        name: "dig".to_string(),
        r#type: 1,
        description: "Try your luck!".to_string(),
    };

    let response = register_command(&request).await;

    match response {
        Ok(()) => println!("ok"),

        Err(_) => println!("err"),
    }
}
