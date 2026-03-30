```rust
use icedxdy::widget::{alert, text};

alert::alert()
    .icon(icon::circle_check_big().size(16))
    .title(text::small("Account updated successfully").semibold())
    .description(text::small("Your profile information has been saved. Changes will be reflected immediately."))
```
