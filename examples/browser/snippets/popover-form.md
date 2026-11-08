```rust
use icedxdy::widget::{button, column, field, input, popover};

popover(
    |p| p.trigger(button("Open Popover").style(button::Outline)),
    |_| {
        column![
            popover::header("Dimensions", "Set the dimensions for the layer."),
            column![
                field(input("").default_value("100%").width(Fill))
                    .label("Width")
                    .orientation(Orientation::Horizontal),
                field(input("").default_value("25px").width(Fill))
                    .label("Height")
                    .orientation(Orientation::Horizontal),
            ]
            .spacing(16),
        ]
        .spacing(10)
        .into()
    },
)
.align(popover::Align::Start)
.width(256)
```
