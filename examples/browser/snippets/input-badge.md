```rust
use icedxdy::widget::{badge, field, input, row, space, text, Slot};

field(
    input("https://api.example.com/webhook")
        .width(Fill),
)
.label(Slot::from(
    row![
        text::label("Webhook URL"),
        space::horizontal(),
        badge("Beta").style(badge::Secondary),
    ]
    .width(Fill),
))
.width(320)
```
