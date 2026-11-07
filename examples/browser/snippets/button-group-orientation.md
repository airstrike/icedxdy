```rust
use icedxdy::widget::button;

button_group![
    button(icon::plus()).square().style(button::Outline).on_press(Message::Increase),
    button(icon::minus()).square().style(button::Outline).on_press(Message::Decrease),
]
.orientation(button::group::Vertical)
```
