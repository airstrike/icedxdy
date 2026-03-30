use icedxdy::Component;
use icedxdy::widget::{alert, text};

use crate::icon;
use super::Message;

pub fn view<'a>() -> Component<'a, Message> {
    alert::alert()
        .icon(icon::circle_check_big().size(16))
        .title(text::small("Account updated successfully").semibold().letter_spacing(-0.025))
        .description(text::small("Your profile information has been saved. Changes will be reflected immediately."))
        .into()
}
