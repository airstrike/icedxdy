```rust
use icedxdy::widget::sidebar;

sidebar::floating(
    column![
        sidebar::header([
            sidebar::menu([
                sidebar::menu_item([
                    sidebar::menu_button("Select Workspace")
                        .on_press(Message::OpenWorkspaces)
                        .into(),
                ])
                .into(),
            ])
            .into(),
        ]),
    ]
    .width(sidebar::WIDTH),
)
.height(288)
```
