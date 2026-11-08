```rust
use icedxdy::widget::{input, input_group};

input_group(input("Search..."))
    .addon(input_group::addon(icon::search()))
    .addon(input_group::addon("12 results").align(input_group::Align::InlineEnd))
```
