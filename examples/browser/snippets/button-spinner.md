```rust
use icedxdy::widget::{button, row, spinner};
use icedxdy::theme;

row![
    button![spinner(), "Generating"]
        .style(theme::button::outline),
    button!["Downloading", spinner()]
        .style(theme::button::secondary),
]
```
