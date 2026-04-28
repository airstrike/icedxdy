```rust
use icedxdy::theme::button::{Outline, Secondary};
use icedxdy::widget::{button, row, spinner};

row![
    button![spinner(), "Generating"]
        .style(Outline),
    button!["Downloading", spinner()]
        .style(Secondary),
]
```
