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
            menu::group([
                menu::item("Team").into(),
                menu::sub(
                    "Invite users",
                    column![
                        menu::item("Email"),
                        menu::item("Message"),
                        menu::separator(),
                        menu::item("More..."),
                    ],
                ),
                menu::item("New Team").shortcut("⌘+T").into(),
            ]),
            menu::separator(),
            menu::group([
                menu::item("GitHub").into(),
                menu::item("Support").into(),
                menu::item("API").disabled(true).into(),
            ]),
            menu::separator(),
            menu::item("Log out").shortcut("⇧⌘Q"),
        ]
        .into()
    },
)
.align(dropdown_menu::Align::Start)
.width(160)
```
