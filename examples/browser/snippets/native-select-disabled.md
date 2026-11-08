```rust
use icedxdy::widget::native_select;

native_select([
    native_select::item("", "Disabled"),
    native_select::item("apple", "Apple"),
    native_select::item("banana", "Banana"),
    native_select::item("blueberry", "Blueberry"),
])
.disabled(true)
```
