```rust
use icedxdy::widget::{avatar, row};

row![
    avatar(Some(airstrike_handle())).fallback("AT").size(avatar::Sm),
    avatar(Some(airstrike_handle())).fallback("AT"),
    avatar(Some(airstrike_handle())).fallback("AT").size(avatar::Lg),
]
.spacing(8)
.align_y(iced::Center)
```
