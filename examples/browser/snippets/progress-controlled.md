```rust
use iced::widget::slider;
use icedxdy::widget::{column, progress};

column![
    progress(self.value).width(320),
    slider(0.0..=100.0, self.value, Message::ValueChanged)
        .step(1.0)
        .width(320),
]
.spacing(16)
```
