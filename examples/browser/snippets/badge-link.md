```rust
use icedxdy::widget::badge;

badge(row!["Open Link", icon::arrow_up_right()].spacing(4))
    .on_press(Message::Open)
```
