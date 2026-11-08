```rust
use icedxdy::widget::{button, row, sheet};

row![
    profile(sheet::Side::Top, "Top"),
    profile(sheet::Side::Right, "Right"),
    profile(sheet::Side::Bottom, "Bottom"),
    profile(sheet::Side::Left, "Left"),
]

fn profile(side: sheet::Side, label: &'static str) -> Slot<'static, Message> {
    sheet(move |s| s.trigger(button(label).style(button::Outline)), edit_profile)
        .side(side)
        .into()
}
```
