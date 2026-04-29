```rust
use icedxdy::theme;
use icedxdy::widget::{input, text};

column![
    row![
        text::small("Required Field").medium(),
        text::small("*").style(theme::text::Destructive),
    ]
    .spacing(2),
    input("This field is required", &self.value)
        .on_input(Message::Changed),
    text::muted("This field must be filled out."),
]
```
