```rust
use icedxdy::widget::{column, item};

fn inbox<'a>(title: &'a str, description: &'a str) -> item::Item<'a, Message> {
    item::item([
        item::media(icon::inbox()).style(item::Icon).into(),
        item::content([item::title(title).into(), item::description(description).into()]).into(),
    ])
}

column![
    inbox("Default Variant", "Transparent background with no border."),
    inbox("Outline Variant", "Outlined style with a visible border.").style(item::Outline),
    inbox("Muted Variant", "Muted background for secondary content.").style(item::Muted),
]
.spacing(24)
.width(Fill)
.max_width(448.0)
```
