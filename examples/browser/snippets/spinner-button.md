```rust
use icedxdy::widget::{button, column, spinner};

column![
    button![spinner(), "Loading..."].size(Size::Sm).disabled(true),
    button![spinner(), "Please wait"].size(Size::Sm).style(button::Outline).disabled(true),
    button![spinner(), "Processing"].size(Size::Sm).style(button::Secondary).disabled(true),
]
.spacing(16)
.align_x(Center)
```
