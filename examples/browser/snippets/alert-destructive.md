```rust
use icedxdy::widget::{alert, text};
use icedxdy::theme;

alert::alert()
    .icon(icon::circle_alert().size(16))
    .title(text::small("Payment failed").semibold()
        .style(theme::text::destructive))
    .description(text::small("Your payment could not be processed. Please check your payment method and try again.")
        .style(theme::text::destructive))
    .destructive()
```
