```rust
use icedxdy::theme;
use icedxdy::widget::{column, row, switch, text};

row![
    column![
        text::small("Accept terms and conditions")
            .medium()
            .style(theme::text::Destructive),
        text::muted("You must accept the terms and conditions to continue."),
    ]
    .spacing(2)
    .width(iced::Fill),
    switch(self.is_on)
        .on_toggle(Message::Toggled)
        .style(theme::switch::Invalid),
]
.spacing(8)
.width(384)
```
