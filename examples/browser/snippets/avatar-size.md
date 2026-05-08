```rust
use icedxdy::widget::{avatar, row};

row![
    avatar(Some(shadcn_handle())).fallback("CN").size(avatar::Sm),
    avatar(Some(shadcn_handle())).fallback("CN"),
    avatar(Some(shadcn_handle())).fallback("CN").size(avatar::Lg),
]
.spacing(8)
.align_y(iced::Center)
```
