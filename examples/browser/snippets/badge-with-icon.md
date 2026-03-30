```rust
use icedxdy::widget::badge;

badge(row![icon::badge_check(), "Verified"])
    .variant(badge::Variant::Secondary)

badge(row!["Bookmark", icon::bookmark()])
    .variant(badge::Variant::Outline)
```
