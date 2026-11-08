```rust
use icedxdy::widget::toggle_group;

toggle_group(|| {
    [
        toggle_group::item("all", "All"),
        toggle_group::item("missed", "Missed"),
    ]
})
.default_value(["all"])
.style(toggle_group::Outline)
```
