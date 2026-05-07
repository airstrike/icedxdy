## Usage

```rust
use icedxdy::widget::sidebar;
```

```rust
// Fixed sidebar (flush against window edge)
sidebar::fixed(content)

// Floating sidebar (margin, rounded corners, border, shadow)
sidebar::floating(content)

// Inset sidebar (content area carries the chrome via main_styled)
sidebar::inset(content)
```

Build the sidebar body using the suite helpers:

```rust
sidebar::fixed(
    column![
        sidebar::header([sidebar::menu_button("Acme Inc.").into()]),
        sidebar::content([
            sidebar::group([
                sidebar::group_label("Projects").into(),
                sidebar::group_content([
                    sidebar::menu([
                        sidebar::menu_item([
                            sidebar::menu_button("Dashboard")
                                .active(true)
                                .on_press(Message::Navigate)
                                .into(),
                        ])
                        .into(),
                    ])
                    .into(),
                ])
                .into(),
            ])
            .into(),
        ]),
    ]
    .width(sidebar::WIDTH),
)
```
