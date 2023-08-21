use requests::RegistrationRequest;

pub mod requests;

pub fn register_command(req: &RegistrationRequest) {
    println!("registering interaction: {}", req.name);
}

