```rust
use icedxdy::widget::native_select;

native_select([
    native_select::item("", "Select status"),
    native_select::item("todo", "Todo"),
    native_select::item("in-progress", "In Progress"),
    native_select::item("done", "Done"),
    native_select::item("cancelled", "Cancelled"),
])
```
