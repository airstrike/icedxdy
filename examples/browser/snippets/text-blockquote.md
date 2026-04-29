```rust
use icedxdy::widget::{rule, text};

// blockquote's left border is paragraph-level decoration that the Text
// widget doesn't model — compose row + Rule at the call site.
row![
    rule::vertical(2),
    text::blockquote(
        "\"After all,\" he said, \"everyone enjoys a good joke, so it's only \
         fair that they should pay for the privilege.\""
    ),
]
.spacing(24)
```
