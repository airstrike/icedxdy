```rust
use icedxdy::widget::{button, column, item};

column![
    item::item([
        item::content([
            item::title("Basic Item").into(),
            item::description("A simple item with title and description.").into(),
        ])
        .into(),
        item::actions([button("Action").style(button::Outline).size(Size::Sm).into()]),
    ])
    .style(item::Outline),
    item::item([
        item::media(icon::badge_check().size(20)).into(),
        item::content([item::title("Your profile has been verified.").into()]).into(),
        item::actions([icon::chevron_right().size(16).into()]),
    ])
    .style(item::Outline)
    .size(item::Size::Sm),
]
.spacing(24)
.max_width(448.0)
```
