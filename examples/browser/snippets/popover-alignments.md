```rust
use icedxdy::widget::{button, popover, row, text};

row![
    aligned("Start", popover::Align::Start, "Aligned to start"),
    aligned("Center", popover::Align::Center, "Aligned to center"),
    aligned("End", popover::Align::End, "Aligned to end"),
]
.spacing(24)

fn aligned<'a>(label: &'a str, align: popover::Align, content: &'a str) -> Slot<'a, Message> {
    popover(
        move |p| p.trigger(button(label).size(Size::Sm).style(button::Outline)),
        move |_| text(content).into(),
    )
    .align(align)
    .width(160)
    .into()
}
```
