```rust
use icedxdy::widget::{checkbox, row, text};

row![
    checkbox(self.is_checked).on_toggle(Message::ToggleTerms),
    text::small("Accept terms and conditions").medium(),
]
.spacing(8)
.width(224)
```
