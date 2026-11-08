```rust
use icedxdy::widget::{context_menu, menu};

context_menu(
    |_| area(),
    |_| {
        menu::group([
            menu::item("Back").into(),
            menu::item("Forward").disabled(true).into(),
            menu::item("Reload").into(),
        ])
    },
)
```
