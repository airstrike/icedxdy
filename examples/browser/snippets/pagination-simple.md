```rust
use icedxdy::widget::pagination;

pagination([
    pagination::link("1").into(),
    pagination::link("2").active(true).into(),
    pagination::link("3").into(),
    pagination::link("4").into(),
    pagination::link("5").into(),
])
```
