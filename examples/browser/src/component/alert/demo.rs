use icedxdy::Component;
use icedxdy::widget::{alert, text};

use crate::icon;
use super::Message;

pub fn view<'a>() -> Component<'a, Message> {
    icedxdy::widget::column([
        alert::alert()
            .icon(icon::circle_check_big().size(16))
            .title(text::small("Payment successful").semibold().letter_spacing(-0.025))
            .description(text::small("Your payment of $29.99 has been processed. A receipt has been sent to your email address."))
            .into(),
        alert::alert()
            .icon(icon::info().size(16))
            .title(text::small("New feature available").semibold().letter_spacing(-0.025))
            .description(text::small("We've added dark mode support. You can enable it in your account settings."))
            .into(),
    ])
    .spacing(16)
    .into()
}
