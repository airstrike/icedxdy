```rust
use icedxdy::Size::{Lg, Sm};
use icedxdy::widget::{button, column};

column([
    button::group![
        button::outline("Small").size(Sm).on_press(Message::Small),
        button::outline("Button").size(Sm).on_press(Message::Button),
        button::outline("Group").size(Sm).on_press(Message::Group),
        button::icon![plus].size(Sm).on_press(Message::Add),
    ]
    .into(),
    button::group![
        button::outline("Default").on_press(Message::Default),
        button::outline("Button").on_press(Message::Button),
        button::outline("Group").on_press(Message::Group),
        button::icon![plus].on_press(Message::Add),
    ]
    .into(),
    button::group![
        button::outline("Large").size(Lg).on_press(Message::Large),
        button::outline("Button").size(Lg).on_press(Message::Button),
        button::outline("Group").size(Lg).on_press(Message::Group),
        button::icon![plus].size(Lg).on_press(Message::Add),
    ]
    .into(),
])
.spacing(32)
```
