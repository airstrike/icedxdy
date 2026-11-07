```rust
use icedxdy::widget::{button, row};

row![
    button_group![
        button(icon::arrow_left()).square().style(button::Outline).on_press(Message::GoBack),
    ],
    button_group![
        button("Archive").style(button::Outline).on_press(Message::Archive),
        button("Report").style(button::Outline).on_press(Message::Report),
    ],
    button_group![
        button("Snooze").style(button::Outline).on_press(Message::Snooze),
        button(icon::ellipsis()).square().style(button::Outline).on_press(Message::MoreOptions),
    ],
]
.spacing(8)
```
