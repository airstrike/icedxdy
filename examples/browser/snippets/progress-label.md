```rust
use icedxdy::widget::{column, progress, row, text};

column([
    row()
        .push(text::small("Upload progress").into())
        .push(iced_widget::space::horizontal().into())
        .push(text::small("66%").into())
        .into(),
    progress(66.0).width(320).into(),
])
.spacing(8)
.width(320)
```
