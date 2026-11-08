```rust
use icedxdy::Vertical;
use icedxdy::widget::toggle_group;

toggle_group(formatting)
    .mode(toggle_group::Mode::Multiple)
    .default_value(["bold", "italic"])
    .orientation(Vertical)
    .spacing(4)
```
