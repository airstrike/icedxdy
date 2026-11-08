```rust
use icedxdy::widget::{button, popover};

popover(
    |p| p.trigger(button("Open Popover").style(button::Outline)),
    |_| popover::header("Dimensions", "Set the dimensions for the layer."),
)
.align(popover::Align::Start)
```
