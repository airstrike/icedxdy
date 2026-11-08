```rust
use icedxdy::widget::{button, column, input, popover, row, text};

popover(
    |p| p.trigger(button("Open popover").style(button::Outline)),
    |_| {
        column![
            column![
                text("Dimensions").medium(),
                text::muted("Set the dimensions for the layer."),
            ]
            .spacing(8),
            column![
                dimension("Width", "100%"),
                dimension("Max. width", "300px"),
                dimension("Height", "25px"),
                dimension("Max. height", "none"),
            ]
            .spacing(8),
        ]
        .spacing(16)
        .into()
    },
)
.width(320)

fn dimension<'a>(label: &'a str, value: &'a str) -> Slot<'a, Message> {
    row![
        text(label).width(FillPortion(1)),
        input("").default_value(value).width(FillPortion(2)),
    ]
    .spacing(16)
    .align_y(Center)
    .into()
}
```
