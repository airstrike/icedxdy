```rust
use icedxdy::widget::{button, dialog};

dialog(
    |d| d.trigger(button("No Close Button").style(button::Outline)),
    |_| dialog::header(
        "No Close Button",
        "This dialog doesn't have a close button in the top-right corner.",
    ),
)
.show_close_button(false)
```
