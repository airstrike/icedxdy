```rust
use icedxdy::widget::breadcrumb;

breadcrumb([
    breadcrumb::link("Home", Message::GoHome),
    breadcrumb::link("Components", Message::GoComponents),
    breadcrumb::page("Breadcrumb"),
])
.separator(|| icon::chevron_right().size(14).into())
```
