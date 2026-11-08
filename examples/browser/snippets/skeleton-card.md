```rust
use icedxdy::card;
use icedxdy::widget::{aspect_ratio, column, row, skeleton, space};

/// A 16px line spanning `part` of `whole` of its parent's width.
fn line<'a>(part: u16, whole: u16) -> Slot<'a, Message> {
    row![
        skeleton().height(16).width(FillPortion(part)),
        space::horizontal().width(FillPortion(whole - part)),
    ]
    .width(Fill)
    .into()
}

card(
    column![
        card::header(line(2, 3)).description(line(1, 2)),
        card::content(aspect_ratio(skeleton().width(Fill).height(Fill), 16.0 / 9.0)),
    ]
    .spacing(Gap::G4)
    .padding(Padding::ZERO.top(Gap::G4).bottom(Gap::G4)),
)
.width(320.0)
```
