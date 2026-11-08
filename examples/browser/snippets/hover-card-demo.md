```rust
use std::time::Duration;

use icedxdy::widget::{button, column, container, hover_card, text};

hover_card(
    button("Hover Here").style(button::Link),
    column![
        text("@nextjs").semibold(),
        text("The React Framework – created and maintained by @vercel."),
        container(text::muted("Joined December 2021").size(text::size::XS))
            .padding([4, 0, 0, 0]),
    ]
    .spacing(2),
)
.open_delay(Duration::from_millis(10))
.close_delay(Duration::from_millis(100))
```
