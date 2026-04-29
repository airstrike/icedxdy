```rust
use icedxdy::theme;
use icedxdy::theme::tweak;
use icedxdy::widget::{input, text};

column![
    text::small("Invalid Input").semibold(),
    input("Error", &self.value)
        .on_input(Message::Changed)
        .style(
            theme::text_input::Default
                + tweak::border::Color(tweak::Token::Destructive)
                + tweak::focus(
                    tweak::border::Color(tweak::Token::Destructive)
                        + tweak::Ring {
                            color: tweak::Token::Destructive,
                            width: 3.0,
                            offset: 0.0,
                        },
                ),
        ),
    text::muted("This field contains validation errors."),
]
```
