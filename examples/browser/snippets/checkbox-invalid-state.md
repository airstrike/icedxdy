```rust
use icedxdy::theme;
use icedxdy::widget::{checkbox, column, field, text};

column![
    field::field(
        checkbox(self.is_checked)
            .on_toggle(Message::Toggle)
            .style(theme::checkbox::Invalid),
    )
    .orientation(field::Orientation::Horizontal)
    .label(text::label("Accept terms and conditions").style(theme::text::Destructive)),
]
.width(224)
```
