```rust
use icedxdy::widget::{checkbox, column, field};

column![
    field::field(checkbox(self.is_checked).on_toggle(Message::Toggle))
        .orientation(field::Orientation::Horizontal)
        .label("Accept terms and conditions"),
]
.width(224)
```
