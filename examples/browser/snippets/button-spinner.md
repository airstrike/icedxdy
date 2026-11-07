```rust
use icedxdy::widget::{button, row, spinner};

row![
    button![spinner(), "Generating"]
        .style(button::Outline)
        .disabled(true),
    button!["Downloading", spinner()]
        .style(button::Secondary)
        .disabled(true),
]
```
