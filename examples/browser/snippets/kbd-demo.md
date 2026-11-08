```rust
use icedxdy::widget::{column, kbd, kbd_group, text};

column![
    kbd_group![kbd("⌘"), kbd("⇧"), kbd("⌥"), kbd("⌃")],
    kbd_group![kbd("Ctrl"), text::small("+"), kbd("B")],
]
.spacing(16)
.align_x(Center)
```
