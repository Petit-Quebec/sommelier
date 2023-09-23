/*!
 * Implementation for returning an error.
 */

use crate::handlers::Handler;
use crate::{InteractionRequest, InteractionResponse};

pub struct ErrorHandler;

impl Handler for ErrorHandler {
    fn handle_application_command(&self, _: &InteractionRequest) -> InteractionResponse {
        InteractionResponse::new().message("Unknown command...")
    }
}
