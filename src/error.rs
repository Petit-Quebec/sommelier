/*!
 * Implementation for returning an error.
 */

use crate::Handler;
use discord_interaction::{Request, Response};

pub struct ErrorHandler;

impl Handler for ErrorHandler {
    fn handle_application_command(&self, _: &Request) -> Response {
        Response::message().content("Unknown command...").into()
    }
}
