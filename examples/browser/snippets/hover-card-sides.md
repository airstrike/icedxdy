```rust
use std::time::Duration;

use icedxdy::widget::{button, column, hover_card, row, text};

row![
    side("Left", hover_card::Side::Left),
    side("Top", hover_card::Side::Top),
    side("Bottom", hover_card::Side::Bottom),
    side("Right", hover_card::Side::Right),
]
.spacing(8)

fn side<'a>(label: &'a str, side: hover_card::Side) -> Slot<'a, Message> {
    hover_card(
        button(label).style(button::Outline),
        column![
            text("Hover Card").medium(),
            text(format!(
                "This hover card appears on the {} side of the trigger.",
                label.to_lowercase()
            )),
        ]
        .spacing(4),
    )
    .side(side)
    .open_delay(Duration::from_millis(100))
    .close_delay(Duration::from_millis(100))
    .into()
}
```
