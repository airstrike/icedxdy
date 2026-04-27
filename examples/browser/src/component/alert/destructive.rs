use icedxdy::Component;
use icedxdy::widget::alert;

use crate::icon;
use super::Message;

pub fn view<'a>() -> Component<'a, Message> {
    alert::alert()
        .icon(icon::circle_alert())
        .title("Payment failed")
        .description("Your payment could not be processed. Please check your payment method and try again.")
        .destructive()
        .into()
}
