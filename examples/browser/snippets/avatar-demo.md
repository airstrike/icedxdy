```rust
use icedxdy::widget::{avatar, row};

row![
    avatar(Some(shadcn_handle())).fallback("CN"),
    avatar::badge(
        avatar(Some(evilrabbit_handle())).fallback("ER"),
        avatar::dot(),
    ),
    avatar::group([
        avatar(Some(shadcn_handle())).fallback("CN"),
        avatar(Some(maxleiter_handle())).fallback("LR"),
        avatar(Some(evilrabbit_handle())).fallback("ER"),
    ])
    .count("+3"),
]
.spacing(24)
.align_y(iced::Center)
```
