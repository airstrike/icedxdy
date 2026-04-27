use icedxdy::Component;
use icedxdy::widget::{alert, text};
use icedxdy::theme;

use crate::icon;
use super::Message;

pub fn view<'a>() -> Component<'a, Message> {
    alert::alert()
        .icon(icon::circle_alert().size(16))
        .title(text::small("Payment failed").semibold().letter_spacing(-0.025).style(theme::text::Destructive))
        .description(text::small("Your payment could not be processed. Please check your payment method and try again.").style(theme::text::Destructive))
        .destructive()
        .into()
}
