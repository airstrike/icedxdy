```rust
use icedxdy::widget::{button, collapsible, column, container, space, text};

collapsible(|c| {
    let chevron = if c.is_open() { icon::chevron_up() } else { icon::chevron_down() };

    container(column![
        c.trigger(
            button!["Product details", space::horizontal(), chevron]
                .width(Fill)
                .style(button::Ghost),
        ),
        c.content(column![
            text::small("This panel can be expanded or collapsed to reveal additional content."),
            button("Learn More").size(Size::Xs),
        ]),
    ])
    .style(if c.is_open() { muted } else { plain })
    .into()
})
```
