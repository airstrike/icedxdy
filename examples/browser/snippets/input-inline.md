```rust
use icedxdy::widget::{button, input};

row![
    input("Search...")
        .width(Fill),
    button("Search")
        .on_press(Message::Search),
]
.spacing(8)
.width(400)
```
