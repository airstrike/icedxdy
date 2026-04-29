```rust
use icedxdy::theme;
use icedxdy::widget::{input, text};

column![
    row![
        text::small("Required Field").semibold(),
        text::small(" *").style(theme::text::Destructive),
    ],
    input("This field is required", &self.value)
        .on_input(Message::Changed),
    text::muted("This field must be filled out."),
]
```
