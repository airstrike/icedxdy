```rust
use icedxdy::widget::{button, row};

row![
    button::group![
        button::icon![plus].on_press(Message::Add),
    ],
    button::group![
        button::outline("Add").on_press(Message::Add),
        button::outline("Remove").on_press(Message::Remove),
    ],
]
.spacing(8)
```
