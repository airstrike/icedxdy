```rust
use icedxdy::widget::{checkbox, column, field, text};

column![
    field::field(checkbox(self.terms_1).on_toggle(Message::ToggleTerms1))
        .orientation(field::Orientation::Horizontal)
        .label("Accept terms and conditions"),
    field::field(checkbox(self.terms_2).on_toggle(Message::ToggleTerms2))
        .orientation(field::Orientation::Horizontal)
        .label("Accept terms and conditions")
        .description("By clicking this checkbox, you agree to the terms."),
    field::field(checkbox(false))
        .orientation(field::Orientation::Horizontal)
        .label(text::muted("Enable notifications")),
    field::field(checkbox(self.notif_2).on_toggle(Message::ToggleNotif))
        .orientation(field::Orientation::Horizontal)
        .label("Enable notifications")
        .description("You can enable or disable notifications at any time.")
        .card(self.notif_2)
        .on_press(Message::ToggleNotif(!self.notif_2)),
]
.spacing(20)
.width(384)
```
