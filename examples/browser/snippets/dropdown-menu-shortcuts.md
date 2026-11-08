```rust
use icedxdy::widget::{button, column, dropdown_menu, menu};

dropdown_menu(
    |m| m.trigger(button("Open").style(button::Outline)),
    |_| {
        column![
            menu::group([
                menu::label("My Account"),
                menu::item("Profile").shortcut("⇧⌘P").into(),
                menu::item("Billing").shortcut("⌘B").into(),
                menu::item("Settings").shortcut("⌘S").into(),
            ]),
            menu::separator(),
            menu::item("Log out").shortcut("⇧⌘Q"),
        ]
        .width(Fill)
        .into()
    },
)
```
