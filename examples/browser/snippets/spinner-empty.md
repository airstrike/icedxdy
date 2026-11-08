```rust
use icedxdy::widget::{button, empty, spinner};

empty::empty()
    .media(empty::media(spinner()).style(empty::Icon))
    .title("Processing your request")
    .description("Please wait while we process your request. Do not refresh the page.")
    .content(button("Cancel").style(button::Outline).size(Size::Sm))
```
