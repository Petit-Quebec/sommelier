/*!
 * Implementation of "hello" command.
 */

use crate::GuildMember;
use crate::InteractionCallbackData;

pub fn hello(member: &Option<GuildMember>) -> InteractionCallbackData {
    let message = match member {
        Some(m) => {
            let name = match &m.nick {
                Some(n) => n.to_owned(),

                None => "friend".to_string(),
            };

            format!("Hello, {}!", name)
        }

        None => "Hello, stranger!".to_string(),
    };

    InteractionCallbackData {
        content: Some(message),
    }
}
