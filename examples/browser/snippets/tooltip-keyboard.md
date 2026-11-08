```rust
use icedxdy::widget::{button, kbd, row, text, tooltip};

tooltip(
    button(icon::save()).size(Size::Sm).square().style(button::Outline),
    row![text::Text::new("Save Changes"), kbd("S").style(kbd::Tooltip)]
        .spacing(6)
        .align_y(Center),
)
```
