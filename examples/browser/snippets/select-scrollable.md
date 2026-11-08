```rust
use icedxdy::widget::select;

select([
    select::group([
        select::label("North America"),
        select::item("est", "Eastern Standard Time"),
        select::item("cst", "Central Standard Time"),
        // ...
    ]),
    select::group([
        select::label("Europe & Africa"),
        select::item("gmt", "Greenwich Mean Time"),
        // ...
    ]),
    // Asia, Australia & Pacific, South America ...
])
.placeholder("Select a timezone")
.icon(icon::chevron_down)
.width(256)
```
