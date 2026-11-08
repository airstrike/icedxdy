```rust
use icedxdy::widget::pagination;

pagination([
    pagination::previous().icon(icon::chevron_left()).on_press(Message::Previous).into(),
    pagination::link("1").on_press(Message::Page(1)).into(),
    pagination::link("2").active(true).into(),
    pagination::link("3").on_press(Message::Page(3)).into(),
    pagination::ellipsis().icon(icon::ellipsis()).into(),
    pagination::next().icon(icon::chevron_right()).on_press(Message::Next).into(),
])
```
