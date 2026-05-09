```rust
use iced::Fill;
use iced::widget::stack;

use icedxdy::card;
use icedxdy::widget::{aspect_ratio, badge, button, column};
use icedxdy::Gap;

card(column![
    aspect_ratio(stack![image_placeholder(), image_overlay()], 16.0 / 9.0),
    card::header("Design systems meetup")
        .description(
            "A practical talk on component APIs, accessibility, and shipping faster.",
        )
        .action(card::action(badge::secondary("Featured"))),
    card::footer(
        button("View Event")
            .on_press(Message::View)
            .width(Fill)
    ),
]
.spacing(Gap::G4)
.padding(iced::Padding::ZERO.bottom(Gap::G4))
.width(Fill))
.width(384.0)
.clip(true)
```
