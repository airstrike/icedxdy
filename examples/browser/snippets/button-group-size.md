```rust
use icedxdy::Size::{Lg, Sm};
use icedxdy::widget::{button, column};

column([
    button_group![
        button("Small").style(button::Outline).size(Sm).on_press(Message::Small),
        button("Button").style(button::Outline).size(Sm).on_press(Message::Button),
        button("Group").style(button::Outline).size(Sm).on_press(Message::Group),
        button(icon::plus()).square().style(button::Outline).size(Sm).on_press(Message::Add),
    ]
    .into(),
    button_group![
        button("Default").style(button::Outline).on_press(Message::Default),
        button("Button").style(button::Outline).on_press(Message::Button),
        button("Group").style(button::Outline).on_press(Message::Group),
        button(icon::plus()).square().style(button::Outline).on_press(Message::Add),
    ]
    .into(),
    button_group![
        button("Large").style(button::Outline).size(Lg).on_press(Message::Large),
        button("Button").style(button::Outline).size(Lg).on_press(Message::Button),
        button("Group").style(button::Outline).size(Lg).on_press(Message::Group),
        button(icon::plus()).square().style(button::Outline).size(Lg).on_press(Message::Add),
    ]
    .into(),
])
.spacing(32)
```
