```rust
use icedxdy::widget::{breadcrumb, text};

breadcrumb([
    breadcrumb::link(text::muted("Home"), Message::GoHome),
    breadcrumb::ellipsis(Some(Message::ToggleMenu)),
    breadcrumb::link(text::muted("Components"), Message::GoComponents),
    breadcrumb::page("Breadcrumb"),
])
.separator(|| icon::chevron_right().size(14).into())
```
