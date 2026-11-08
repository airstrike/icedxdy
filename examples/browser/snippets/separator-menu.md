```rust
use icedxdy::Vertical;
use icedxdy::widget::{column, row, separator, text};

fn menu_item<'a>(title: &'a str, description: &'a str) -> Slot<'a, Message> {
    column![
        text::small(title).medium(),
        text::xs(description).style(theme::text::Muted),
    ]
    .spacing(4)
    .into()
}

row![
    menu_item("Settings", "Manage preferences"),
    separator().orientation(Vertical),
    menu_item("Account", "Profile & security"),
    separator().orientation(Vertical),
    menu_item("Help", "Support & docs"),
]
.spacing(16)
.align_y(Center)
```
