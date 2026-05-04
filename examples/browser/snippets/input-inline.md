```rust
use icedxdy::widget::{button, input};

row![
    input("Search...", &self.value)
        .on_input(Message::Changed)
        .width(Fill),
    button("Search")
        .on_press(Message::Search),
]
.spacing(8)
.width(400)
```
