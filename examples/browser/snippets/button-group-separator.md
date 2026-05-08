```rust
use icedxdy::Size::Sm;
use icedxdy::widget::button;

button::group![
    button::secondary("Copy").size(Sm).on_press(Message::Copy),
    button::group::separator(),
    button::secondary("Paste").size(Sm).on_press(Message::Paste),
]
```
