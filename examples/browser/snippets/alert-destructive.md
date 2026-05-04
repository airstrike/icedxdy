```rust
use icedxdy::widget::alert;

alert::destructive()
    .icon(icon::circle_alert())
    .title("Payment failed")
    .description("Your payment could not be processed. Please check your payment method and try again.")
    .max_width(448)
```
