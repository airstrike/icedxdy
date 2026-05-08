```rust
use icedxdy::Size::{Lg, Sm};
use icedxdy::widget::{button, button_group, column};

column([
    button_group([
        button::outline("Small").size(Sm).into(),
        button::outline("Button").size(Sm).into(),
        button::outline("Group").size(Sm).into(),
        button::icon![plus].size(Sm).into(),
    ])
    .into(),
    button_group([
        button::outline("Default").into(),
        button::outline("Button").into(),
        button::outline("Group").into(),
        button::icon![plus].into(),
    ])
    .into(),
    button_group([
        button::outline("Large").size(Lg).into(),
        button::outline("Button").size(Lg).into(),
        button::outline("Group").size(Lg).into(),
        button::icon![plus].size(Lg).into(),
    ])
    .into(),
])
.spacing(32)
```
