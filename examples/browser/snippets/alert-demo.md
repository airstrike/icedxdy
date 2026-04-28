```rust
use iced::Fill;
use icedxdy::widget::{alert, column};

column![
    alert::alert()
        .icon(icon::circle_check_big())
        .title("Payment successful")
        .description("Your payment of $29.99 has been processed. A receipt has been sent to your email address."),
    alert::alert()
        .icon(icon::info())
        .title("New feature available")
        .description("We've added dark mode support. You can enable it in your account settings."),
]
.spacing(16)
.width(Fill)
```
