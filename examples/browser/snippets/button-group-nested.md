```rust
use icedxdy::widget::{button, row};

row![
    button_group![
        button(icon::plus()).square().style(button::Outline).on_press(Message::Add),
    ],
    button_group![
        button("Add").style(button::Outline).on_press(Message::Add),
        button("Remove").style(button::Outline).on_press(Message::Remove),
    ],
]
.spacing(8)
```
