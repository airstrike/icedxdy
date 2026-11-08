```rust
use icedxdy::widget::{button, collapsible, column, row, space, text};

collapsible(|c| {
    column![
        row![
            text::small("Order #4189").semibold(),
            space::horizontal(),
            c.trigger(button(icon::chevrons_up_down()).square().style(button::Ghost)),
        ],
        bordered(row![text::muted("Status"), space::horizontal(), text::small("Shipped").medium()]),
        c.content(
            column![
                detail("Shipping address", "100 Market St, San Francisco"),
                detail("Items", "2x Studio Headphones"),
            ]
            .spacing(8),
        ),
    ]
    .spacing(8)
    .into()
})
.width(350)
// Controlled, as in shadcn's demo:
// .open(self.is_open).on_open_change(Message::OpenChanged)
```
