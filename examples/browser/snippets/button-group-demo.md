```rust
use icedxdy::widget::{button, row};

row![
    button::group![
        button::icon![arrow_left].on_press(Message::GoBack),
    ],
    button::group![
        button::outline("Archive").on_press(Message::Archive),
        button::outline("Report").on_press(Message::Report),
    ],
    button::group![
        button::outline("Snooze").on_press(Message::Snooze),
        button::icon![ellipsis].on_press(Message::MoreOptions),
    ],
]
.spacing(8)
```
