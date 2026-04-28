```rust
use icedxdy::theme::button::Outline;
use icedxdy::widget::{button, row};

row![
    button![icon::git_branch(), "New Branch"]
        .style(Outline),
    button!["Fork", icon::git_branch()]
        .style(Outline),
]
```
