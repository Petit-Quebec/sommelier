use contractor::register_command;
use contractor::requests::RegistrationRequest;

fn main() {
    let request = RegistrationRequest { 
        name: "dig".to_string(), 
        r#type: 1, 
        description: "Try your luck!".to_string() };

    register_command(&request);
}
