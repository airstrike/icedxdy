```rust
use icedxdy::widget::{checkbox, column, field, text};

column![
    field(checkbox(self.terms_1).on_toggle(Message::ToggleTerms1))
        .orientation(field::Horizontal)
        .label("Accept terms and conditions"),
    field(checkbox(self.terms_2).on_toggle(Message::ToggleTerms2))
        .orientation(field::Horizontal)
        .label("Accept terms and conditions")
        .description("By clicking this checkbox, you agree to the terms."),
    field(checkbox(false))
        .orientation(field::Horizontal)
        .label(text::muted("Enable notifications")),
    field(checkbox(self.notif_2).on_toggle(Message::ToggleNotif))
        .orientation(field::Horizontal)
        .label("Enable notifications")
        .description("You can enable or disable notifications at any time.")
        .card(self.notif_2)
        .on_press(Message::ToggleNotif(!self.notif_2)),
]
.spacing(20)
.width(384)
```
