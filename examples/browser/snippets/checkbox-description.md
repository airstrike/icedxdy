```rust
use icedxdy::widget::{checkbox, column, field};

column![
    field(checkbox(self.is_checked).on_toggle(Message::ToggleTerms))
        .orientation(field::Orientation::Horizontal)
        .label("Accept terms and conditions")
        .description("By clicking this checkbox, you agree to the terms and conditions."),
]
.width(288)
```
