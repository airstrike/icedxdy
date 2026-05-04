use icedxdy::Component;
use icedxdy::Size;
use icedxdy::widget::{alert, button};

use super::Message;

pub fn view<'a>() -> Component<'a, Message> {
    // shadcn `alert-action.tsx` caps the alert at `max-w-md` (448px).
    alert()
        .title("Dark mode is now available")
        .description("Enable it under your profile settings to get started.")
        .action(button("Enable").size(Size::Xs).on_press(Message::Pressed))
        .max_width(448)
        .into()
}
