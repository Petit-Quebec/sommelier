/*!
 * Script to set global commands.
 */

use contractor::register_command;
use contractor::request_types::RegistrationRequest;

#[tokio::main]
async fn main() {
    let request = RegistrationRequest {
        name: "deedee".to_string(),
        r#type: 1,
        description: "what's a deedee?".to_string(),
    };

    let response = register_command(&request).await;

    match response {
        Ok(()) => println!("Successfully created DEEDEE command."),

        Err(err) => println!("Failed to create DEEDEE command: {}", err),
    }
}
