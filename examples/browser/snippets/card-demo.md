```rust
use iced::Alignment::Center;
use iced::Fill;

use icedxdy::card;
use icedxdy::widget::{button, column, field, input, row, space, text};

let password_label = row![
    text::label("Password"),
    space::horizontal(),
    button::link("Forgot your password?").on_press(Message::Forgot),
]
.align_y(Center)
.width(Fill);

card(column![
    card::header("Login to your account")
        .description("Enter your email below to login to your account")
        .action(card::action(
            button::link("Sign Up").on_press(Message::SignUp),
        )),
    card::content(
        column![
            field(
                input("m@example.com", &self.email)
                    .on_input(Message::EmailChanged)
                    .width(Fill),
            )
            .label("Email"),
            field(
                input("", &self.password)
                    .on_input(Message::PasswordChanged)
                    .width(Fill),
            )
            .label(password_label),
        ]
        .spacing(24)
    ),
    card::footer(
        column![
            button("Login").on_press(Message::Login).width(Fill),
            button::outline("Login with Google")
                .on_press(Message::Google)
                .width(Fill),
        ]
        .spacing(8)
    ),
]
.spacing(16)
.padding(iced::Padding::ZERO.top(16).bottom(16)))
.max_width(384)
```
