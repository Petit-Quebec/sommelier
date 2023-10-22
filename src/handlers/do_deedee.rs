/*!
 * Implementation of "deedee" command.
 */

use crate::handlers::Handler;
use discord_interaction::{Request, Response};

pub struct DeedeeHandler;

impl Handler for DeedeeHandler {
    fn handle_application_command(&self, _: &Request) -> Response {
        Response::message().content("mega doo doo").into()
    }
}
