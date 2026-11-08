```rust
use icedxdy::widget::{button, column, dropdown_menu, menu, row, text};

dropdown_menu(
    |m| m.trigger(button("Actions").style(button::Outline)),
    |_| {
        column![
            menu::group([
                menu::item(row![icon::pencil(), text("Edit")].spacing(6)).into(),
                menu::item(row![icon::share(), text("Share")].spacing(6)).into(),
            ]),
            menu::separator(),
            menu::item(row![icon::trash_2(), text("Delete")].spacing(6))
                .style(menu::Destructive),
        ]
        .into()
    },
)
```
