/*!
 * Primes the application by registering the initial global request. This script should only need
 * to be run once.
 */

use contractor::register_command;
use contractor::request_types::RegistrationRequest;

#[tokio::main]
async fn main() {
    let request = RegistrationRequest {
        name: "dig".to_string(),
        r#type: 1,
        description: "Try your luck!".to_string(),
    };

    let response = register_command(&request).await;

    match response {
        Ok(()) => println!("Successfully created DIG command."),

        Err(err) => println!("Failed to create DIG command: {}", err),
    }
}
