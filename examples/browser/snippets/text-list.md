```rust
use icedxdy::widget::{column, container, text};

// nova `typography-list.tsx` is `ml-6 list-disc [&>li]:mt-2` — 24px left
// indent + 8px gap between items. We pad the column 24px on the left.
container(
    column![
        text::list("1st level of puns: 5 gold coins"),
        text::list("2nd level of jokes: 10 gold coins"),
        text::list("3rd level of one-liners : 20 gold coins"),
    ]
    .spacing(8)
)
.padding([0, 0, 0, 24])
```
