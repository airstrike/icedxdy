```rust
use icedxdy::widget::avatar;

avatar::group([
    avatar(Some(shadcn_handle())).fallback("CN"),
    avatar(Some(maxleiter_handle())).fallback("LR"),
    avatar(Some(evilrabbit_handle())).fallback("ER"),
])
```
