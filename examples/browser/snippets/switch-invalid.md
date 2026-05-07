```rust
use icedxdy::theme;
use icedxdy::widget::{column, row, switch, text};

row![
    column![
        text("Accept terms and conditions"),
        text::muted("You must accept the terms and conditions to continue."),
    ]
    .spacing(2)
    .width(iced::Fill),
    switch(self.is_on)
        .on_toggle(Message::Toggled)
        .style(theme::switch::Invalid),
]
.spacing(16)
```
