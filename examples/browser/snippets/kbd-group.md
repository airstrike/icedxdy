```rust
use icedxdy::widget::{kbd, kbd_group, row, text};

row![
    text::muted("Use"),
    kbd_group![kbd("Ctrl + B"), kbd("Ctrl + K")],
    text::muted("to open the command palette"),
]
.spacing(4)
.align_y(Center)
```
