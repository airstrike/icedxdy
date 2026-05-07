```rust
use icedxdy::widget::{checkbox, field};

field(checkbox(self.is_checked).on_toggle(Message::Toggle))
    .orientation(field::Horizontal)
    .label("Accept terms and conditions")
    .width(224)
```
