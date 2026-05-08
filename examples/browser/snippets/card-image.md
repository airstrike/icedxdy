```rust
use iced::Fill;
use iced::widget::stack;

use icedxdy::card;
use icedxdy::spacing;
use icedxdy::widget::{aspect_ratio, badge, button, column};

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
.spacing(spacing::GAP_4)
.padding(iced::Padding::ZERO.bottom(spacing::GAP_4)))
.width(384.0)
.clip(true)
```
