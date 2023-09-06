/*!
 * Implementation of "deedee" command.
 */

use crate::app::InteractionCallbackData;
use crate::app::InteractionData;

pub fn deedee(_: &InteractionData) -> InteractionCallbackData {
    InteractionCallbackData {
        content: Some("mega doo doo".to_string()),
    }
}
