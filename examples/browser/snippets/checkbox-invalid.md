```rust
use icedxdy::theme;
use icedxdy::widget::{checkbox, row, text};

row![
    checkbox(self.is_checked)
        .on_toggle(Message::ToggleTerms)
        .style(theme::checkbox::Invalid),
    text::label("Accept terms and conditions")
        .style(theme::text::Destructive),
]
.spacing(8)
.width(224)
```
