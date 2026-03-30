```rust
use icedxdy::widget::badge;

row![
    badge("Default"),
    badge("Secondary").variant(badge::Variant::Secondary),
    badge("Destructive").variant(badge::Variant::Destructive),
    badge("Outline").variant(badge::Variant::Outline),
    badge("Ghost").variant(badge::Variant::Ghost),
]
```
