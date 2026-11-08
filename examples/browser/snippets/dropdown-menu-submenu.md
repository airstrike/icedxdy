```rust
use icedxdy::widget::{button, dropdown_menu, menu};

dropdown_menu(
    |m| m.trigger(button("Open").style(button::Outline)),
    |_| {
        menu::group([
            menu::item("Team").into(),
            menu::sub(
                "Invite users",
                [
                    menu::item("Email").into(),
                    menu::item("Message").into(),
                    menu::sub(
                        "More options",
                        [
                            menu::item("Calendly").into(),
                            menu::item("Slack").into(),
                            menu::separator(),
                            menu::item("Webhook").into(),
                        ],
                    ),
                    menu::separator(),
                    menu::item("Advanced...").into(),
                ],
            ),
            menu::item("New Team").shortcut("⌘+T").into(),
        ])
    },
)
```
