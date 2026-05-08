```rust
use icedxdy::widget::{button, button_group, row};

// Three nested ButtonGroups inside an outer row produce three visual
// clusters separated by 8px (matches shadcn's `gap-2` cascade triggered
// by `[data-slot=button-group]` children). The third cluster's icon
// button is shadcn's DropdownMenu trigger — rendered here without the
// menu since DropdownMenu is out of scope.
row![
    button_group([
        button::icon![arrow_left]
            .on_press(Message::GoBack)
            .into(),
    ]),
    button_group([
        button::outline("Archive")
            .on_press(Message::Archive)
            .into(),
        button::outline("Report")
            .on_press(Message::Report)
            .into(),
    ]),
    button_group([
        button::outline("Snooze")
            .on_press(Message::Snooze)
            .into(),
        button::icon![ellipsis]
            .on_press(Message::MoreOptions)
            .into(),
    ]),
]
.spacing(8)
```
