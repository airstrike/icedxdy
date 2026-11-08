```rust
use icedxdy::widget::{input, input_group};

input_group(input("https://x.com/shadcn")).addon(
    input_group::addon(input_group::button(icon::copy()).square().on_press(Message::Copy))
        .align(input_group::Align::InlineEnd),
)
```
