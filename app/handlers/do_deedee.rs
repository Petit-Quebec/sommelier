/*!
 * Implementation of "deedee" command.
 */

use crate::InteractionCallbackData;
use crate::InteractionData;

pub fn deedee(_: &InteractionData) -> InteractionCallbackData {
    InteractionCallbackData {
        content: Some("mega doo doo".to_string()),
    }
}
