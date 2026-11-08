```rust
use icedxdy::widget::{column, toggle_group};

fn sides<'a>() -> [toggle_group::Item<'a, Message>; 4] {
    ["Top", "Bottom", "Left", "Right"].map(|side| toggle_group::item(side.to_lowercase(), side))
}

column![
    toggle_group(sides).default_value(["top"]).size(Size::Sm).style(toggle_group::Outline),
    toggle_group(sides).default_value(["top"]).style(toggle_group::Outline),
]
.spacing(16)
```
