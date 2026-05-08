```rust
use icedxdy::widget::button;

button::group![
    button::icon![plus].on_press(Message::Increase),
    button::icon![minus].on_press(Message::Decrease),
]
.orientation(button::group::Vertical)
```
