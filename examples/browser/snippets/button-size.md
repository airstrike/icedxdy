```rust
use icedxdy::widget::{button, row};
use icedxdy::Size::*;

row![
    row![
        button("Extra Small").style(button::Outline).size(Xs),
        button(icon::arrow_up_right()).square().style(button::Outline).size(Xs),
    ],
    row![
        button("Small").style(button::Outline).size(Sm),
        button(icon::arrow_up_right()).square().style(button::Outline).size(Sm),
    ],
    row![
        button("Default").style(button::Outline),
        button(icon::arrow_up_right()).square().style(button::Outline),
    ],
    row![
        button("Large").style(button::Outline).size(Lg),
        button(icon::arrow_up_right()).square().style(button::Outline).size(Lg),
    ],
]
```
