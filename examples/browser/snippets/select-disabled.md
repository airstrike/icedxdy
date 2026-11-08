```rust
use icedxdy::widget::select;

select([select::group([
    select::item("apple", "Apple"),
    select::item("banana", "Banana"),
    select::item("blueberry", "Blueberry"),
    select::item("grapes", "Grapes").disabled(true),
    select::item("pineapple", "Pineapple"),
])])
.placeholder("Select a fruit")
.icon(icon::chevron_down)
.disabled(true)
.width(192)
```
