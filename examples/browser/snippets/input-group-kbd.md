```rust
use icedxdy::widget::{input, input_group, kbd};

input_group(input("Search..."))
    .addon(input_group::addon(icon::search()))
    .addon(input_group::addon(kbd("⌘K")).align(input_group::Align::InlineEnd))
```
