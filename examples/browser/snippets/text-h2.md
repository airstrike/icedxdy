```rust
use icedxdy::widget::{column, rule, text};

// nova `typography-h2.tsx` is `border-b pb-2` — compose the underline at
// the call site with a 1px horizontal rule.
column![
    text::h2("The People of the Kingdom"),
    rule::horizontal(1),
]
.spacing(8)
```
