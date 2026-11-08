```rust
use icedxdy::widget::{column, separator, text};

column![
    column![
        text::label("shadcn/ui"),
        text::muted("The Foundation for your Design System"),
    ]
    .spacing(6),
    separator(),
    text::small("A set of beautifully designed components that you can customize, extend, and build on."),
]
.spacing(16)
.max_width(384.0)
```
