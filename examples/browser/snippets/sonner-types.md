```rust
use icedxdy::widget::{button, row, toast};

fn raise<'a>(label: &'a str, toast: toast::Toast<Message>) -> Slot<'a, Message> {
    toast::trigger(button(label).style(button::Outline), toast)
}

row![
    raise("Default", toast::toast("Event has been created")),
    raise("Success", toast::toast("Event has been created").kind(toast::Kind::Success)),
    raise("Info", toast::toast("Be at the area 10 minutes before the event time").kind(toast::Kind::Info)),
    raise("Warning", toast::toast("Event start time cannot be earlier than 8am").kind(toast::Kind::Warning)),
    raise("Error", toast::toast("Event has not been created").kind(toast::Kind::Error)),
]
.spacing(8)
```
