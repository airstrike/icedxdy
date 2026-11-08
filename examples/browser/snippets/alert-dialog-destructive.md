```rust
use icedxdy::theme::tweak::{Alpha, Bg, Fg, Token};
use icedxdy::widget::{alert_dialog, button, column};

alert_dialog(
    |a| a.trigger(button("Delete Chat").style(button::Destructive)),
    |a| {
        column![
            a.header(
                "Delete chat?",
                "This will permanently delete this chat conversation. View Settings delete any memories saved during this chat.",
            )
            .media(icon::trash_2())
            .media_style(alert_dialog::Media + Bg(Token::Destructive) + Alpha(0.1) + Fg(Token::Destructive)),
            a.footer([
                a.cancel(button("Cancel").style(button::Outline)),
                a.action(button("Delete").style(button::Destructive).on_press(Message::Delete)),
            ]),
        ]
        .width(Fill)
        .into()
    },
)
.size(Size::Sm)
```
