```rust
use icedxdy::widget::{button, row};
use icedxdy::Size::*;

row![
    row![
        button::outline("Extra Small").size(Xs),
        button::icon(icon::arrow_up_right()).size(Xs),
    ],
    row![
        button::outline("Small").size(Sm),
        button::icon(icon::arrow_up_right()).size(Sm),
    ],
    row![
        button::outline("Default"),
        button::icon(icon::arrow_up_right()),
    ],
    row![
        button::outline("Large").size(Lg),
        button::icon(icon::arrow_up_right()).size(Lg),
    ],
]
```
