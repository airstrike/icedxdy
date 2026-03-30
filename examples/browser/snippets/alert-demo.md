```rust
use icedxdy::widget::{alert, text};

column![
    alert::alert()
        .icon(icon::circle_check_big().size(16))
        .title(text::small("Payment successful").semibold())
        .description(text::small("Your payment of $29.99 has been processed. A receipt has been sent to your email address.")),
    alert::alert()
        .icon(icon::info().size(16))
        .title(text::small("New feature available").semibold())
        .description(text::small("We've added dark mode support. You can enable it in your account settings.")),
]
```
