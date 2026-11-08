```rust
use icedxdy::widget::{item, spinner, text};

item::item([
    item::media(spinner()).into(),
    item::content([item::title("Processing payment...").into()]).into(),
    item::content([text::small("$100.00").into()]).width(Shrink).into(),
])
.style(item::Muted)
```
