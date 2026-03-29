```rust
use icedxdy::widget::{button, input, row};

row![
    input::input("Email", &self.email)
        .on_input(Message::EmailChanged),
    button("Subscribe").on_press(Message::Submit),
]
.spacing(8)
```
