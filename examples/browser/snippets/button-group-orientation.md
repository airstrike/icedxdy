```rust
use icedxdy::widget::{button, button_group};

button_group([
    button::icon![plus].into(),
    button::icon![minus].into(),
])
.orientation(button_group::Vertical)
```
