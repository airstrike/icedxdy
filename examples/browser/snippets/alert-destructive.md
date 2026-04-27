```rust
use icedxdy::widget::alert;

alert::alert()
    .icon(icon::circle_alert())
    .title("Payment failed")
    .description("Your payment could not be processed. Please check your payment method and try again.")
    .destructive()
```
