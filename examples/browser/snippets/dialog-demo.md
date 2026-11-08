```rust
use icedxdy::widget::{button, column, dialog, field, input};

dialog(
    |d| d.trigger(button("Open Dialog").style(button::Outline)),
    |d| {
        column![
            dialog::header(
                "Edit profile",
                "Make changes to your profile here. Click save when you're done.",
            ),
            field(input("").default_value("Pedro Duarte")).label("Name"),
            field(input("").default_value("@peduarte")).label("Username"),
            dialog::footer([
                d.close(button("Cancel").style(button::Outline)),
                d.close(button("Save changes")),
            ]),
        ]
        .spacing(16)
        .into()
    },
)
```
