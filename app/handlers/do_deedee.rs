/*!
 * Implementation of "deedee" command.
 */

use crate::handlers::Handler;
use crate::{InteractionData, InteractionResponse};

pub struct DeedeeHandler;

impl Handler for DeedeeHandler {
    fn handle_application_command(&self, _: &InteractionData) -> InteractionResponse {
        InteractionResponse::new().message("mega doo doo")
    }
}
