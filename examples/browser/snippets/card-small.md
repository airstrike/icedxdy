```rust
use iced::Fill;

use icedxdy::card;
use icedxdy::widget::{button, column, text};
use icedxdy::Size;

card(column![
    card::header("Small Card")
        .description("This card uses the small size variant."),
    card::content(text::p(
        "The card component supports a size prop that can be set to \"sm\" for a more compact appearance."
    )),
    card::footer(
        button::outline("Action")
            .size(Size::Sm)
            .on_press(Message::Action)
            .width(Fill)
    ),
]
.spacing(16)
.padding(iced::Padding::ZERO.top(16).bottom(16)))
.max_width(384)
```
