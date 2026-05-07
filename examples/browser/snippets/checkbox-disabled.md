```rust
use icedxdy::widget::{checkbox, row, text};

// No `.on_toggle(...)` ⇒ disabled (matches iced's input convention).
// `text::muted` approximates nova's `opacity-50` cascade on the label.
row![
    checkbox(false),
    text::muted("Enable notifications"),
]
.spacing(8)
.width(224)
```
