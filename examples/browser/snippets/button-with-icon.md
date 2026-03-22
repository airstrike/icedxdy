```rust
use icedxdy::widget::{button, row};
use icedxdy::theme;

row![
    button![icon::git_branch(), "New Branch"]
        .style(theme::button::outline),
    button!["Fork", icon::git_branch()]
        .style(theme::button::outline),
]
```
