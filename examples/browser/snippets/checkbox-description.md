```rust
use icedxdy::widget::{checkbox, column, row, text};

row![
    checkbox(self.is_checked).on_toggle(Message::ToggleTerms),
    column![
        text::label("Accept terms and conditions"),
        text::muted("By clicking this checkbox, you agree to the terms and conditions."),
    ]
    .spacing(2),
]
.spacing(8)
.width(288)
```
