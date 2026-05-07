```rust
use icedxdy::widget::{checkbox, column, row, text};

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
    row![
        checkbox(self.notif_2).on_toggle(Message::ToggleNotif),
        column![
            text::label("Enable notifications"),
            text::muted("You can enable or disable notifications at any time."),
        ]
        .spacing(2),
    ]
    .spacing(8),
]
.spacing(20)
.width(384)
```
