```rust
use icedxdy::widget::{button, dropdown_menu, menu};

// `status`, `activity` and `panel` live in your state.
dropdown_menu(
    |m| m.trigger(button("Open").style(button::Outline)),
    move |_| {
        menu::group([
            menu::label("Appearance"),
            menu::checkbox_item("Status Bar", status, Message::StatusBar).into(),
            menu::checkbox_item("Activity Bar", activity, Message::ActivityBar)
                .disabled(true)
                .into(),
            menu::checkbox_item("Panel", panel, Message::Panel).into(),
        ])
    },
)
.width(160)
```
