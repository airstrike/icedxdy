```rust
use icedxdy::widget::{field, select};

field(
    select([select::group([
        select::item("apple", "Apple"),
        select::item("banana", "Banana"),
        select::item("blueberry", "Blueberry"),
    ])])
    .placeholder("Select a fruit")
    .icon(icon::chevron_down)
    .style(select::Invalid)
    .width(Fill),
)
.label("Fruit")
.error("Please select a fruit.")
.width(192)
```
