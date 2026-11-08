```rust
use icedxdy::widget::{column, row, switch, text};

row![
    column![
        text::label("Share across devices"),
        text::muted(
            "Focus is shared across devices, and turns off when you leave the app.",
        ),
    ]
    .spacing(2)
    .width(iced::Fill),
    switch(),
]
.spacing(8)
.width(384)
```
