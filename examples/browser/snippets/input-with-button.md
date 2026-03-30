```rust
use icedxdy::widget::{button, input};

row![
    input::input("Search...", &self.value)
        .on_input(Message::Changed),
    button("Search")
        .on_press(Message::Search),
]
```
