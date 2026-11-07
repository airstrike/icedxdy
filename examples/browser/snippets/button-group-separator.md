```rust
use icedxdy::Size::Sm;
use icedxdy::widget::button;

button_group![
    button("Copy").style(button::Secondary).size(Sm).on_press(Message::Copy),
    button::group::separator(),
    button("Paste").style(button::Secondary).size(Sm).on_press(Message::Paste),
]
```
