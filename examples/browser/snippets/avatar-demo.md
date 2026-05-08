```rust
use icedxdy::widget::{avatar, row};

row![
    avatar(Some(airstrike_handle())).fallback("AT"),
    avatar::badge(
        avatar(Some(hecrj_handle())).fallback("HR"),
        avatar::dot(),
    ),
    avatar::group([
        avatar(Some(airstrike_handle())).fallback("AT"),
        avatar(Some(hecrj_handle())).fallback("HR"),
        avatar(Some(iced_handle())).fallback("IC"),
    ])
    .count("+3"),
]
.spacing(24)
.align_y(iced::Center)
```
