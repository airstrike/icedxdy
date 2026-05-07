```rust
use icedxdy::theme;
use icedxdy::widget::{checkbox, row, text};

row![
    checkbox(self.is_checked)
        .on_toggle(Message::Toggled)
        .style(theme::checkbox::Invalid),
    text("Accept terms and conditions"),
]
.spacing(8)
```
