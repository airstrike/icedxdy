```rust
use icedxdy::widget::{badge, spinner};

badge::destructive(row![spinner(), "Deleting"].spacing(4))

badge::secondary(row!["Generating", spinner()].spacing(4))
```
