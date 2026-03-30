```rust
use icedxdy::widget::badge;

row![
    badge("Badge"),
    badge("Secondary").variant(badge::Variant::Secondary),
    badge("Destructive").variant(badge::Variant::Destructive),
    badge("Outline").variant(badge::Variant::Outline),
]
```
