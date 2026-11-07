use icedxdy::Slot;
use icedxdy::widget::alert;

use super::Message;
use crate::icon;

pub fn view<'a>() -> Slot<'a, Message> {
    // shadcn `alert-basic.tsx` caps the alert at `max-w-md` (448px).
    alert()
        .icon(icon::circle_check_big())
        .title("Account updated successfully")
        .description(
            "Your profile information has been saved. Changes will be reflected immediately.",
        )
        .max_width(448)
        .into()
}
