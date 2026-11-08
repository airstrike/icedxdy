```rust
use icedxdy::widget::{button, column, dropdown_menu, menu};

dropdown_menu(
    |m| m.trigger(button("Open").style(button::Outline)),
    |_| {
        column![
            menu::group([
                menu::label("My Account"),
                menu::item("Profile").into(),
                menu::item("Billing").into(),
                menu::item("Settings").into(),
            ]),
            menu::separator(),
            menu::item("GitHub"),
            menu::item("Support"),
            menu::item("API").disabled(true),
        ]
        .into()
    },
)
```
