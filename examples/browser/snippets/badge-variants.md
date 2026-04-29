```rust
use icedxdy::widget::badge;

row![
    badge("Default"),
    badge::secondary("Secondary"),
    badge::destructive("Destructive"),
    badge::outline("Outline"),
    badge::ghost("Ghost"),
]
.spacing(8)
```
