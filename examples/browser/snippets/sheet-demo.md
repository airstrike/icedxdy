```rust
use icedxdy::widget::{button, column, field, input, sheet};

sheet(
    |s| s.trigger(button("Open").style(button::Outline)),
    |s| {
        column![
            sheet::header(
                "Edit profile",
                "Make changes to your profile here. Click save when you're done.",
            ),
            column![
                field(input("").default_value("Pedro Duarte")).label("Name"),
                field(input("").default_value("@peduarte")).label("Username"),
            ]
            .spacing(24)
            .padding([0, 16]),
            sheet::footer([
                s.close(button("Save changes")),
                s.close(button("Close").style(button::Outline)),
            ]),
        ]
        .into()
    },
)
```
