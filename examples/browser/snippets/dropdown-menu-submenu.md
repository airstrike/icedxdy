```rust
use icedxdy::widget::{button, column, dropdown_menu, menu};

dropdown_menu(
    |m| m.trigger(button("Open").style(button::Outline)),
    |_| {
        menu::group([
            menu::item("Team").into(),
            menu::sub(
                "Invite users",
                column![
                    menu::item("Email"),
                    menu::item("Message"),
                    menu::sub(
                        "More options",
                        column![
                            menu::item("Calendly"),
                            menu::item("Slack"),
                            menu::separator(),
                            menu::item("Webhook"),
                        ],
                    ),
                    menu::separator(),
                    menu::item("Advanced..."),
                ],
            ),
            menu::item("New Team").shortcut("⌘+T").into(),
        ])
    },
)
```
