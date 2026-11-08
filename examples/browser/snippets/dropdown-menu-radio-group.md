```rust
use icedxdy::widget::{button, dropdown_menu, menu};

// `position` lives in your state.
dropdown_menu(
    |m| m.trigger(button("Open").style(button::Outline)),
    move |_| {
        menu::group([
            menu::label("Panel Position"),
            menu::radio_item("Top", "top", Some(position), Message::Position).into(),
            menu::radio_item("Bottom", "bottom", Some(position), Message::Position).into(),
            menu::radio_item("Right", "right", Some(position), Message::Position).into(),
        ])
    },
)
.width(128)
```
