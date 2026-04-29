```rust
use icedxdy::widget::{container, text};
use icedxdy::theme::container::CardMuted;
use icedxdy::{radius, Size::Sm};

// bg-muted + padding + small rounded corners come from the container;
// the Text helper sets font + size + weight.
container(text::inline_code("@radix-ui/react-alert-dialog"))
    .padding([3, 5])
    .style(CardMuted + radius(Sm))
```
