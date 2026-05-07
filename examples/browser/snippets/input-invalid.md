```rust
use icedxdy::theme;
use icedxdy::widget::{column, field, input};

column![
    field::field(
        input("Error", &self.value)
            .on_input(Message::Changed)
            .width(Fill)
            .style(theme::text_input::Invalid),
    )
    .label("Invalid Input")
    .description("This field contains validation errors."),
]
.width(320)
```
