```rust
use icedxdy::Size;
use icedxdy::widget::{alert, button};

alert::alert()
    .title("Dark mode is now available")
    .description("Enable it under your profile settings to get started.")
    .action(button::button("Enable").size(Size::Xs).on_press(Message::Pressed))
```
