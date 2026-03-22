```rust
use icedxdy::widget::{button, row, spinner};

row![
    button::outline![spinner(), "Generating"],
    button::secondary!["Downloading", spinner()],
]
```
