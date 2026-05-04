```rust
use icedxdy::rich_text;
use icedxdy::theme::tweak::Token;
use icedxdy::widget::rich_text::span;
use icedxdy::widget::{input, text};

column![
    rich_text![
        span("Required Field ").size(text::size::SM).medium(),
        span("*").size(text::size::SM).color(Token::Destructive),
    ],
    input("This field is required", &self.value)
        .on_input(Message::Changed)
        .width(Fill),
    text::muted("This field must be filled out."),
]
.spacing(8)
.width(320)
```
