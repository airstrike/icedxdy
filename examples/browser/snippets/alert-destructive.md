```rust
use iced::widget::container;
use icedxdy::widget::alert;

container(
    alert::alert()
        .icon(icon::circle_alert())
        .title("Payment failed")
        .description("Your payment could not be processed. Please check your payment method and try again.")
        .destructive(),
)
.max_width(448)
```
