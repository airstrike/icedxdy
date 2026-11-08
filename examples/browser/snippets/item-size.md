```rust
use icedxdy::widget::{column, item};

column![
    inbox("Default Size", "The standard size for most use cases.").style(item::Outline),
    inbox("Small Size", "A compact size for dense layouts.")
        .style(item::Outline)
        .size(item::Size::Sm),
    inbox("Extra Small Size", "The most compact size available.")
        .style(item::Outline)
        .size(item::Size::Xs),
]
.spacing(24)
.width(Fill)
.max_width(448.0)
```
