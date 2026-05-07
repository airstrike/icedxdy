```rust
use icedxdy::widget::{checkbox, column, field, row, text};

column![
    row![
        checkbox(self.terms_1).on_toggle(Message::ToggleTerms1),
        text::label("Accept terms and conditions"),
    ]
    .spacing(8),
    row![
        checkbox(self.terms_2).on_toggle(Message::ToggleTerms2),
        column![
            text::label("Accept terms and conditions"),
            text::muted("By clicking this checkbox, you agree to the terms."),
        ]
        .spacing(2),
    ]
    .spacing(8),
    row![
        // No `.on_toggle(...)` ⇒ disabled.
        checkbox(false),
        text::muted("Enable notifications"),
    ]
    .spacing(8),
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
