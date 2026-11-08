```rust
use icedxdy::widget::{alert_dialog, button, column};

alert_dialog(
    |a| a.trigger(button("Show Dialog").style(button::Outline)),
    |a| {
        column![
            a.header(
                "Allow accessory to connect?",
                "Do you want to allow the USB accessory to connect to this device?",
            )
            .media(icon::bluetooth()),
            a.footer([
                a.cancel(button("Don't allow").style(button::Outline)),
                a.action(button("Allow")),
            ]),
        ]
        .width(Fill)
        .into()
    },
)
.size(Size::Sm)
```
