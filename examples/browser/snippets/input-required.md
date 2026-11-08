```rust
use icedxdy::rich_text;
use icedxdy::theme::tweak::Token;
use icedxdy::widget::rich_text::span;
use icedxdy::widget::{field, input, text, Slot};

field(
    input("This field is required")
        .width(Fill),
)
.label(Slot::from(rich_text![
    span("Required Field ").size(text::size::SM).medium(),
    span("*").size(text::size::SM).color(Token::Destructive),
]))
.description("This field must be filled out.")
.width(320)
```
