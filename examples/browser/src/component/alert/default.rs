use iced::widget::container;
use icedxdy::Component;
use icedxdy::widget::alert;

use crate::icon;
use super::Message;

pub fn view<'a>() -> Component<'a, Message> {
    // shadcn `alert-basic.tsx` caps the alert at `max-w-md` (448px).
    container(
        alert::alert()
            .icon(icon::circle_check_big())
            .title("Account updated successfully")
            .description("Your profile information has been saved. Changes will be reflected immediately."),
    )
    .max_width(448)
    .into()
}
