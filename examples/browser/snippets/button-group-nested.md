```rust
use icedxdy::widget::{button, button_group, row};

// Nest inner ButtonGroups inside an outer one to introduce 8px spacing
// between sub-clusters (matches shadcn's `gap-2` cascade triggered by a
// `[data-slot=button-group]` child selector).
row![
    button_group([
        button::icon![plus].into(),
    ]),
    button_group([
        button::outline("Add").into(),
        button::outline("Remove").into(),
    ]),
]
.spacing(8)
```
