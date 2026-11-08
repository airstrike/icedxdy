```rust
use icedxdy::widget::select;

select([
    select::group([
        select::label("Fruits"),
        select::item("apple", "Apple"),
        select::item("banana", "Banana"),
        select::item("blueberry", "Blueberry"),
    ]),
    select::separator(),
    select::group([
        select::label("Vegetables"),
        select::item("carrot", "Carrot"),
        select::item("broccoli", "Broccoli"),
        select::item("spinach", "Spinach"),
    ]),
])
.placeholder("Select a fruit")
.icon(icon::chevron_down)
.width(192)
```
