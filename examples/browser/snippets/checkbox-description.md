```rust
use icedxdy::widget::{checkbox, field};

field(checkbox().default_checked(true))
    .orientation(field::Horizontal)
    .label("Accept terms and conditions")
    .description("By clicking this checkbox, you agree to the terms and conditions.")
    .width(288)
```
