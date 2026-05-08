```rust
use icedxdy::Size::Sm;
use icedxdy::widget::{button, button_group};

button_group([
    button::secondary("Copy").size(Sm).into(),
    button_group::separator(),
    button::secondary("Paste").size(Sm).into(),
])
```
