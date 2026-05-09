```rust
use iced::Fill;

use icedxdy::card;
use icedxdy::widget::{button, column, text};
use icedxdy::{Gap, Size};

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
.spacing(Gap::G4)
.padding(iced::Padding::ZERO.top(Gap::G4).bottom(Gap::G4))
.width(Fill))
.width(384.0)
```
