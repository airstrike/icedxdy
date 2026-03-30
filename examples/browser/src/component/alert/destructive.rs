use icedxdy::Component;
use icedxdy::widget::{alert, text};
use icedxdy::theme;

use crate::icon;
use super::Message;

pub fn view<'a>() -> Component<'a, Message> {
    alert::alert()
        .icon(icon::circle_alert().size(16))
        .title(text::small("Error").semibold().letter_spacing(-0.025).style(theme::text::destructive))
        .description(text::small("Your session has expired. Please log in again.").style(theme::text::destructive))
        .destructive()
        .into()
}
