```rust
use icedxdy::widget::text;

// nova `typography-list.tsx` is `ml-6 list-disc [&>li]:mt-2` — 24px left
// indent + 8px gap between items. `text::list` packages all of that.
text::list([
    "1st level of puns: 5 gold coins",
    "2nd level of jokes: 10 gold coins",
    "3rd level of one-liners : 20 gold coins",
])
```
