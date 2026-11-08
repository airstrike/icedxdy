```rust
use icedxdy::theme;
use icedxdy::widget::{field, input};

field(
    input("Error")
        .width(Fill)
        .style(theme::text_input::Invalid),
)
.label("Invalid Input")
.description("This field contains validation errors.")
.width(320)
```
