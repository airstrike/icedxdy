use icedxdy::Slot;
use icedxdy::widget::alert;

use crate::Message;
use crate::icon;

pub fn view<'a>() -> Slot<'a, Message> {
    // shadcn `alert-destructive.tsx` caps the alert at `max-w-md` (448px).
    alert::alert()
        .icon(icon::circle_alert())
        .title("Payment failed")
        .description(
            "Your payment could not be processed. Please check your payment method and try again.",
        )
        .style(alert::Destructive)
        .max_width(448)
        .into()
}
