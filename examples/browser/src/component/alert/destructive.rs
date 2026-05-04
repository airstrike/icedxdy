use icedxdy::Component;
use icedxdy::widget::alert;

use crate::icon;
use super::Message;

pub fn view<'a>() -> Component<'a, Message> {
    // shadcn `alert-destructive.tsx` caps the alert at `max-w-md` (448px).
    alert::destructive()
        .icon(icon::circle_alert())
        .title("Payment failed")
        .description("Your payment could not be processed. Please check your payment method and try again.")
        .max_width(448)
        .into()
}
