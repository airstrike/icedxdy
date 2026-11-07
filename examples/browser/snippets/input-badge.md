```rust
use icedxdy::widget::{badge, field, input, row, space, text, Component};

field(
    input("https://api.example.com/webhook").value(&self.value)
        .on_input(Message::Changed)
        .width(Fill),
)
.label(Component::from(
    row![
        text::label("Webhook URL"),
        space::horizontal(),
        badge("Beta").style(badge::Secondary),
    ]
    .width(Fill),
))
.width(320)
```
