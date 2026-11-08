```rust
use icedxdy::widget::{button, column, empty, row};

empty::empty()
    .media(empty::media(icon::folder()).style(empty::Icon))
    .title("No projects yet")
    .description("You haven't created any projects yet. Get started by creating your first project.")
    .content(
        column![
            row![
                button("Create project"),
                button("Import project").style(button::Outline),
            ]
            .spacing(8),
            button!["Learn more", icon::arrow_up_right()]
                .style(button::Link + Fg(Token::MutedForeground)),
        ]
        .spacing(16)
        .align_x(Center),
    )
```
