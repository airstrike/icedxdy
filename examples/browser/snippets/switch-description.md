```rust
use icedxdy::widget::{column, row, switch, text};

row![
    column![
        text("Share across devices"),
        text::muted(
            "Focus is shared across devices, and turns off when you leave the app.",
        ),
    ]
    .spacing(2)
    .width(iced::Fill),
    switch(self.is_on).on_toggle(Message::Toggled),
]
.spacing(16)
```
