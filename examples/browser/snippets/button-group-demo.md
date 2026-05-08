```rust
use icedxdy::widget::{button, button_group};

button_group([
    button::icon![arrow_left]
        .on_press(Message::GoBack)
        .into(),
    button::outline("Archive")
        .on_press(Message::Archive)
        .into(),
    button::outline("Report")
        .on_press(Message::Report)
        .into(),
    button::outline("Snooze")
        .on_press(Message::Snooze)
        .into(),
    button::icon![ellipsis]
        .on_press(Message::MoreOptions)
        .into(),
])
```
