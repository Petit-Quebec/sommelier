/*!
 * Implementation of "deedee" command.
 */

use crate::handlers::Handler;
use crate::{InteractionRequest, InteractionResponse};

pub struct DeedeeHandler;

impl Handler for DeedeeHandler {
    fn handle_application_command(&self, _: &InteractionRequest) -> InteractionResponse {
        InteractionResponse::message()
            .content("mega doo doo")
            .into()
    }
}
