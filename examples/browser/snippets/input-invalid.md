```rust
use icedxdy::theme;
use icedxdy::widget::{input, text};

column![
    text::label("Invalid Input"),
    input("Error", &self.value)
        .on_input(Message::Changed)
        .width(Fill)
        .style(theme::text_input::Invalid),
    text::muted("This field contains validation errors."),
]
.spacing(8)
.width(320)
```
