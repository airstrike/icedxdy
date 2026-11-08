```rust
use icedxdy::widget::{button, item};

item::item([
    item::media(icon::shield_alert()).style(item::Icon).into(),
    item::content([
        item::title("Security Alert").into(),
        item::description("New login detected from unknown device.").into(),
    ])
    .into(),
    item::actions([button("Review").size(Size::Sm).style(button::Outline).into()]),
])
.style(item::Outline)
```
