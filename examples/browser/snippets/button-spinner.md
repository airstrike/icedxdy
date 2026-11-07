```rust
use icedxdy::theme::button::{Outline, Secondary};
use icedxdy::widget::{button, row, spinner};

row![
    button![spinner(), "Generating"]
        .style(Outline)
        .disabled(true),
    button!["Downloading", spinner()]
        .style(Secondary)
        .disabled(true),
]
```
