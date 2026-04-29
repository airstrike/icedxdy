```rust
use icedxdy::theme::button::{Outline, Secondary};
use icedxdy::widget::{button, row, spinner};

// no `.on_press(...)` ⇒ disabled, matches shadcn's `disabled` prop
row![
    button![spinner(), "Generating"]
        .style(Outline),
    button!["Downloading", spinner()]
        .style(Secondary),
]
```
