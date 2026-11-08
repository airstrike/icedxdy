```rust
use icedxdy::widget::{checkbox, column, field, text};

column![
    field(checkbox())
        .orientation(field::Horizontal)
        .label("Accept terms and conditions"),
    field(checkbox().default_checked(true))
        .orientation(field::Horizontal)
        .label("Accept terms and conditions")
        .description("By clicking this checkbox, you agree to the terms."),
    field(checkbox().disabled(true))
        .orientation(field::Horizontal)
        .label(text::muted("Enable notifications")),
    field(checkbox())
        .orientation(field::Horizontal)
        .label("Enable notifications")
        .description("You can enable or disable notifications at any time.")
        .card(),
]
.spacing(20)
.width(384)
```
