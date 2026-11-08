```rust
use icedxdy::widget::{button, toast};

toast::trigger(
    button("Top Left").style(button::Outline),
    toast::toast("Event has been created").position(toast::Position::TopLeft),
)
// ... TopCenter, TopRight, BottomLeft, BottomCenter, BottomRight
```
