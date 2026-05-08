```rust
use icedxdy::widget::{breadcrumb, text};

breadcrumb([
    breadcrumb::link(text::muted("Home"), Message::GoHome),
    breadcrumb::link(text::muted("Components"), Message::GoComponents),
    breadcrumb::page("Breadcrumb"),
])
.separator(|| text::small("•").style(theme::text::Muted).into())
```
