```rust
use icedxdy::widget::{button, column, dialog, input};

dialog(
    |d| d.trigger(button("Share").style(button::Outline)),
    |d| {
        column![
            dialog::header("Share link", "Anyone who has this link will be able to view this."),
            input("").default_value("https://ui.shadcn.com/docs/installation"),
            dialog::footer([d.close(button("Close"))]),
        ]
        .spacing(16)
        .into()
    },
)
```
