```rust
use icedxdy::widget::{button, row, tooltip};

fn side<'a>(label: &'a str, side: tooltip::Side) -> Slot<'a, Message> {
    tooltip(button(label).style(button::Outline), "Add to library")
        .side(side)
        .into()
}

row![
    side("Left", tooltip::Side::Left),
    side("Top", tooltip::Side::Top),
    side("Bottom", tooltip::Side::Bottom),
    side("Right", tooltip::Side::Right),
]
.spacing(8)
```
